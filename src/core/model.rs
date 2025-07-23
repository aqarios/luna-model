use either::Either;
use indexmap::IndexMap;
use itertools::Itertools;
use strum_macros::{Display, EnumString};

use super::constraints::Constraints;
use super::environment::SharedEnvironment;
use super::expression::{ExpressionBaseAdd, ExpressionBaseAdjustment, ExpressionBaseCreation};
use super::solution::OwnedSample;
use super::traits::ContentEquality;
use super::utils::{check_variables_sample, check_variables_sol};
use super::{Expression, ExpressionBase, RcSolution, Sample, Substitution, VarRef, Vtype};
use crate::core::expression::ExpressionEvaluation;
use crate::core::solution::OwnedResult;
use crate::core::writer::ModelWriter;
use crate::errors::{DifferentEnvsErr, EvaluationErr, VariableCreationErr};
use crate::types::{Bias, VarIndex};
#[cfg(feature = "py")]
use pyo3::prelude::*;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

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
    pub constraints: Constraints,
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
            constraints: Constraints::default(),
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
            constraints: Constraints::default(),
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
                None => &format!("x_{}", idx.to_string()),
                Some(names) => &names[idx],
            };
            model
                .environment
                .add_variable(var_name, Some(vtype.unwrap_or(Vtype::Binary)), None)?;
        }

        model.objective.resize(num_variables);
        model
            .objective
            .add_quadratic_from_dense(matrix_flat, num_variables);
        if let Some(off) = offset {
            model.objective.add_offset(off);
        }
        Ok(model)
    }

    pub fn evaluate_solution(&self, sol: RcSolution) -> Result<RcSolution, EvaluationErr> {
        let vars_sol = &sol.variable_names;
        let vars_env = &self.environment.variable_names();
        check_variables_sol(vars_sol, vars_env)?;

        let mut newsol = sol.0.deref().clone();
        for (i, sample) in sol.samples().iter().enumerate() {
            let obj_val = self.objective.evaluate_sample(&sample);
            let constraints = self
                .constraints
                .iter()
                .map(|(_, constr)| constr.evaluate_sample(&sample))
                .collect();
            let variable_bounds = self.environment.borrow().evaluate_bounds::<Sample>(&sample);
            newsol.add_sample_evaluation(i, Some(obj_val), constraints, variable_bounds);
        }
        Ok(RcSolution(newsol.into()))
    }

    pub fn evaluate_sample<'a>(&self, sample: &Sample) -> Result<OwnedResult, EvaluationErr> {
        let (vars_sample, index_map) = match &sample.0 {
            Either::Left(a) => (&a.sol.variable_names, &a.sol.index_map),
            Either::Right(b) => (&b.variable_names, &b.index_map),
        };
        let vars_env = &self.environment.variable_names();
        check_variables_sample(vars_sample, vars_env)?;

        let obj_val = self.objective.evaluate_sample(sample);
        let cf: Vec<_> = self
            .constraints
            .iter()
            .map(|(_, constraint)| {
                let v = constraint.lhs.evaluate_sample(sample);
                constraint.comparator.evaluate(v, constraint.rhs)
            })
            .collect();
        let vf: Vec<_> = self.environment.borrow().evaluate_bounds::<Sample>(&sample);
        let feasible = cf.iter().all(|&b| b) && vf.iter().all(|&b| b);
        let owned_sample_actual = Rc::new(sample.iter().collect());
        let owned_sample =
            OwnedSample::new(vars_sample.to_vec(), owned_sample_actual, index_map.clone());
        Ok(OwnedResult::new(owned_sample, obj_val, cf, vf, feasible))
    }

    pub fn set_sense(&mut self, sense: Sense) -> &mut Self {
        self.sense = sense;
        self
    }

    pub fn violated_constraints(&self, sample: &Sample) -> Constraints {
        let mut index_map = IndexMap::new();
        let mut constraints = Vec::new();
        for (idx, (name, constr)) in self.constraints.iter().enumerate() {
            let v = constr.lhs.evaluate_sample(sample);
            if !constr.comparator.evaluate(v, constr.rhs) {
                index_map.insert(name.to_string(), idx);
                constraints.push(constr.clone())
            }
        }
        Constraints {
            index_map,
            constraints,
        }
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
        self.name == other.name
            && self.environment.is_equal_contents(&other.environment)
            && self.objective.is_equal_contents(&other.objective)
            && self.constraints.is_equal_contents(&other.constraints)
            && self.sense == other.sense
    }
}
