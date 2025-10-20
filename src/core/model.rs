use super::constraints::ConstraintCollection;
use super::environment::SharedEnvironment;
use super::expression::{ExpressionBaseAdd, ExpressionBaseCreation};
use super::solution::result::OwnedResult;
use super::solution::sample::SampleOwned;
use super::solution::sol::Solution;
use super::solution::Sample;
use super::traits::ContentEquality;
use super::utils::{check_variables_sample, check_variables_sol};
use super::{Expression, ExpressionBase, Substitution, VarRef, Vtype};
use crate::core::expression::ExpressionEvaluation;
use crate::core::utils::make_index_map;
use crate::core::writer::ModelWriter;
use crate::core::{Comparator, ConstraintType, ModelSpecs};
use crate::errors::{DifferentEnvsErr, EvaluationErr, VariableCreationErr};
use crate::types::{Bias, VarIndex};
use enumset::EnumSet;
use indexmap::IndexMap;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use strum_macros::{Display, EnumString};
#[cfg(feature = "py")]
use {pyo3::prelude::*, unwind_macros::unwindable, crate::py_bindings::unwind};

/// The default name for a model.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

// we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
#[cfg_attr(
    all(feature = "py", not(feature = "lq")),
    pyclass(eq, eq_int, name = "Sense", module = "aqmodels._core")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(eq, eq_int, name = "Sense", module = "luna_quantum._core")
)]
#[derive(Display, Copy, PartialEq, Hash, Clone, Debug, Eq, EnumString)]
/// Enumeration of optimization senses supported by the optimization system.
///
/// This enum defines the type of optimization used for a model. The type influences
/// the domain and behavior of the model during optimization.
pub enum Sense {
    /// Indicate the objective function to be minimized.
    #[strum(to_string = "Minimize", serialize = "Min")]
    Min,
    /// Indicate the objective function to be maximized.
    #[strum(to_string = "Maximize", serialize = "Max")]
    Max,
}

impl Sense {
    /// Convenience function to check if the sense is `Sense::Min`.
    pub fn is_min(&self) -> bool {
        self == &Self::Min
    }
}

impl Default for Sense {
    fn default() -> Self {
        Self::Min
    }
}

#[cfg(feature = "py")]
#[cfg_attr(feature = "py", pymethods)]
#[cfg_attr(feature = "py", unwindable)]
impl Sense {
    #[getter]
    fn get_name(&self) -> String {
        match &self {
            Self::Min => String::from("Min"),
            Self::Max => String::from("Max"),
        }
    }
    #[getter]
    fn get_value(&self) -> String {
        self.to_string()
    }
}

/// A model describing some function to be optimized (objective) and restrictions
/// on this objective (constraints).
#[derive(Clone)]
pub struct Model {
    /// The name of the model.
    pub name: String,
    /// The environment of the model, constaining the information for each variable
    /// used in both the objective and it's constraints.
    pub environment: SharedEnvironment,
    /// The objective of the model describing some optimization problem. The objective
    /// is an expression that can be linear, quadratic or higher order.
    pub objective: Expression,
    /// The constraints of the model describing the restrictions on the model.
    pub constraints: ConstraintCollection,
    /// The sense of the model, i.e., the direction to be optimized at.
    /// By default is set to `Sense::Min`.
    pub sense: Sense,
}

impl Model {
    /// Deep clone a Model.
    ///
    /// This creates a new Model with a deep clone of the contained data.
    /// The SharedEnvironment is not just an increase of the reference counted environment
    /// but a new SharedEnvironment object with a deep cloned environment having a new
    /// environment id that is guaranteed to be different from all other possibly
    /// exisiting environments.
    pub fn deep_clone(&self) -> Self {
        let new_env = self.environment.deep_clone();
        Self {
            name: self.name.clone(),
            sense: self.sense.clone(),
            objective: self.objective.deep_clone(new_env.clone()),
            constraints: self.constraints.deep_clone(new_env.clone()),
            environment: new_env,
        }
    }
}

impl Model {
    pub fn default() -> Self {
        Self::new(None, None)
    }

    /// Create a new Model using a specifc environment.
    pub fn new_with_env(
        name: Option<String>,
        sense: Option<Sense>,
        env: SharedEnvironment,
    ) -> Self {
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Expression::empty(env.clone()),
            environment: env,
            constraints: ConstraintCollection::default(),
            sense: sense.unwrap_or(Sense::default()),
        }
    }

    /// Create a new Model with a new environment created just for this model.
    pub fn new(name: Option<String>, sense: Option<Sense>) -> Self {
        let rcenv = SharedEnvironment::default();
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Expression::empty(rcenv.clone()),
            environment: rcenv,
            constraints: ConstraintCollection::default(),
            sense: sense.unwrap_or(Sense::default()),
        }
    }

    /// Create a new Model based on a Quadratic Unconstrained Binary Optimization (QUBO)
    /// problem represented as a continuous slice of all values. A new environment is
    /// created based on the size of the QUBO.
    pub fn new_from_dense(
        name: Option<String>,
        vtype: Option<Vtype>,
        matrix_flat: &[Bias],
        num_variables: VarIndex,
        offset: Option<Bias>,
        variable_names: Option<Vec<String>>,
    ) -> Result<Self, VariableCreationErr> {
        let mut model = Model::new(name, Some(Sense::default()));

        for idx in 0..num_variables.into() {
            let var_name = match &variable_names {
                Option::None => &format!("x_{}", idx.to_string()),
                Some(names) => &names[idx],
            };
            model
                .environment
                .add_variable(var_name, Some(vtype.unwrap_or(Vtype::Binary)), None)?;
        }

        // model.objective.resize(num_variables);
        model
            .objective
            .add_quadratic_from_dense(matrix_flat, num_variables);
        if let Some(off) = offset {
            model.objective.add_offset(off);
        }
        Ok(model)
    }

    pub fn evaluate_solution(&self, mut sol: Solution) -> Result<Solution, EvaluationErr> {
        let vars_sol = &sol.variable_names;
        let vars_env = &self.environment.variable_names();
        check_variables_sol(vars_sol, vars_env)?;

        let index_map = make_index_map(sol.varname_to_pos(), &self.environment);

        let mut obj_values = Vec::with_capacity(sol.n_samples);
        let mut constr = Vec::with_capacity(sol.n_samples);
        let mut vb = Vec::with_capacity(sol.n_samples);

        for sample in sol.iter_samples() {
            let obj_val = self
                .objective
                .evaluate_sample(&sample, |var_idx| index_map[&var_idx].into());
            constr.push(
                self.constraints
                    .iter()
                    .map(|(_, constr)| {
                        constr.evaluate_sample(&sample, |var_idx| index_map[&var_idx].into())
                    })
                    .collect(),
            );
            vb.push(
                self.environment
                    .access()
                    .evaluate_bounds(&sample, |var_idx| index_map[&var_idx].into()),
            );
            obj_values.push(obj_val);
        }
        sol.add_eval_data(obj_values, constr, vb);
        Ok(sol)
    }

    pub fn evaluate_sample<'a>(&self, sample: &Sample) -> Result<OwnedResult, EvaluationErr> {
        let sample_var_names = sample.variable_names();
        let env_var_names = &self.environment.variable_names();
        check_variables_sample(&sample_var_names, env_var_names)?;

        let index_map = make_index_map(sample.varname_to_pos(), &self.environment);

        let obj_val = self
            .objective
            .evaluate_sample(sample, |idx| index_map[&idx]);
        let cf: Vec<_> = self
            .constraints
            .iter()
            .map(|(_, constraint)| {
                let v = constraint
                    .lhs
                    .evaluate_sample(sample, |idx| index_map[&idx]);
                constraint.comparator.evaluate(v, constraint.rhs)
            })
            .collect();
        let vf: Vec<_> = self
            .environment
            .access()
            .evaluate_bounds(sample, |idx| index_map[&idx]);
        let feasible = cf.iter().all(|&b| b) && vf.iter().all(|&b| b);
        let owned_sample = SampleOwned::new(
            sample_var_names.to_vec(),
            sample.iter().collect(),
            sample.var_indices(),
        );
        Ok(OwnedResult::new(owned_sample, obj_val, cf, vf, feasible))
    }

    pub fn set_sense(&mut self, sense: Sense) -> &mut Self {
        self.sense = sense;
        self
    }

    pub fn violated_constraints(&self, sample: &Sample) -> ConstraintCollection {
        let var_index_lookup = make_index_map(sample.varname_to_pos(), &self.environment);
        let mut constraints = IndexMap::new();
        for (name, constr) in self.constraints.iter() {
            let v = constr
                .lhs
                .evaluate_sample(sample, |idx| var_index_lookup[&idx].into());
            if !constr.comparator.evaluate(v, constr.rhs) {
                constraints.insert(name.to_string(), constr.clone());
            }
        }
        ConstraintCollection { constraints }
    }

    /// Substitute every occurrence of a variable in the model’s objective and constraint expressions with another expression.
    ///
    /// Given a `Model` instance `self`, this method replaces all occurrences of `target`
    /// with `replacement` for the objective and each constraint.
    /// If any substitution would cross differing environments (e.g. captures from two
    /// different scopes), it returns a `DifferentEnvsError`.
    ///
    /// # Parameters
    /// - `target`: the variable reference to replace
    /// - `replacement`: the expression to insert in place of `target`
    ///
    /// # Returns
    /// - `Ok(())`: Unit type after substitution.
    /// - `Err(DifferentEnvsErr)`: if the environments of `self`, `target`, and `replacement`
    ///    are not compatible
    pub fn substitute(
        &mut self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<(), DifferentEnvsErr> {
        self.objective = (&self.objective).substitute(target, replacement)?;
        self.constraints.substitute(target, replacement)?;
        if !replacement.contains(target) {
            self.environment.remove(target);
        }
        Ok(())
    }

    pub fn vtypes(&self) -> Vec<Vtype> {
        let mut obj_vtypes = self.objective.vtypes();
        let mut constr_vtypes = self.constraints.vtypes();
        obj_vtypes.append(&mut constr_vtypes);
        obj_vtypes.into_iter().unique().collect_vec()
    }

    pub fn num_variables(&self) -> usize {
        let mut vars = self.objective.variables();
        let mut constraint_vars = self
            .constraints
            .iter()
            .map(|(_, c)| c.lhs.variables())
            .flatten()
            .unique()
            .collect_vec();
        // move values in vars
        vars.append(&mut constraint_vars);
        // Get len of all unique vars.
        vars.iter().unique().collect_vec().len()
    }

    pub fn get_specs(&self) -> ModelSpecs {
        let mut vtypes = EnumSet::new();
        for vtype in self.vtypes().iter() {
            vtypes.insert(*vtype);
        }
        let mut constraints = EnumSet::new();
        let mut max_constraint_degress: usize = 0;
        for (_, constr) in self.constraints.iter() {
            max_constraint_degress = max_constraint_degress.max(constr.lhs.degree());
            match constr.comparator {
                Comparator::Eq => constraints.insert(ConstraintType::Equality),
                Comparator::Le => constraints.insert(ConstraintType::LessEqual),
                Comparator::Ge => constraints.insert(ConstraintType::GreaterEqual),
            };
        }

        ModelSpecs::new(
            self.sense,
            vtypes,
            constraints,
            self.objective.degree(),
            max_constraint_degress,
            self.num_variables(),
        )
    }

    pub fn satisfies(&self, specs: ModelSpecs) -> bool {
        self.get_specs().satisfies(specs)
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.environment.id() == other.environment.id()
            && self.objective == other.objective
            && self.constraints == other.constraints
    }
}

impl Debug for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("name", &self.name)
            .field("objective", &self.objective)
            .field("constraints", &self.constraints)
            .field("environment_id", &self.environment.id())
            .finish()
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_model(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Model {
    fn is_equal_contents(&self, other: &Self) -> bool {
        let name_eq = self.name == other.name;
        let env_eq = self.environment.is_equal_contents(&other.environment);
        let obj_eq = self.objective.is_equal_contents(&other.objective);
        let const_eq = self.constraints.is_equal_contents(&other.constraints);
        let sense_eq = self.sense == other.sense;
        name_eq && env_eq && obj_eq && const_eq && sense_eq
    }
}
