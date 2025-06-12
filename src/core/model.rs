use either::Either;
use strum_macros::{Display, EnumString};

use super::constraints::Constraints;
use super::environment::{add_variable, SharedEnvironment};
use super::expression::{ExpressionBaseAdd, ExpressionBaseAdjustment, ExpressionBaseCreation};
use super::solution::OwnedSample;
use super::utils::{check_variables_sample, check_variables_sol};
use super::{Environment, Expression, RcSolution, Sample, Vtype};
use crate::core::expression::ExpressionEvaluation;
use crate::core::solution::OwnedResult;
use crate::core::writer::ModelWriter;
use crate::errors::{EvaluationErr, VariableCreationErr};
use crate::types::{Bias, VarIndex};
#[cfg(feature = "py")]
use pyo3::prelude::*;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

/// The default name for a model.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

#[cfg_attr(
    feature = "py",
    pyclass(eq, eq_int, name = "Sense", module = "aqmodels")
)] // we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
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
        let rcenv = SharedEnvironment::new(Environment::new());
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
            add_variable(
                model.environment.clone(),
                var_name,
                Some(&vtype.unwrap_or(Vtype::Binary)),
                None,
            )?;
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

    pub fn evaluate_solution(
        &self,
        sol: RcSolution,
    ) -> Result<RcSolution, EvaluationErr>
    {
        let vars_sol = &sol.variable_names;
        let vars_env = &self
            .environment
            .borrow()
            .variables
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<String>>();
        check_variables_sol(vars_sol, vars_env)?;

        let mut newsol = sol.0.deref().clone();
        for (i, sample) in sol.samples().iter().enumerate() {
            let obj_val = self.objective.evaluate_sample(&sample);
            let constraints = self
                .constraints
                .iter()
                .map(|constr| constr.evaluate_sample(&sample))
                .collect();
            let variable_bounds = self
                .environment
                .borrow()
                .evaluate_bounds::<Sample>(&sample);
            newsol.add_sample_evaluation(
                i,
                Some(obj_val),
                constraints,
                variable_bounds,
                self.sense.is_min(),
            );
        }
        Ok(RcSolution(newsol.into()))
    }

    pub fn evaluate_sample<'a, AssignmentTypes>(
        &self,
        sample: &Sample,
    ) -> Result<OwnedResult, EvaluationErr>
    {
        let vars_sample = match &sample.0 {
            Either::Left(a) => &a.sol.variable_names,
            Either::Right(b) => &b.variable_names,
        };
        let vars_env = &self
            .environment
            .borrow()
            .variables
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<String>>();
        check_variables_sample(vars_sample, vars_env)?;

        let obj_val = self.objective.evaluate_sample(sample);
        let cf: Vec<_> = self
            .constraints
            .iter()
            .map(|constraint| {
                let v = constraint.lhs.evaluate_sample(sample);
                constraint.comparator.evaluate(v, constraint.rhs)
            })
            .collect();
        let vf: Vec<_> = self
            .environment
            .borrow()
            .evaluate_bounds::<Sample>(&sample);
        let feasible = cf.iter().all(|&b| b) && vf.iter().all(|&b| b);
        let owned_sample_actual = Rc::new(sample.iter().collect());
        let owned_sample = OwnedSample::new(vars_sample.to_vec(), owned_sample_actual);
        Ok(OwnedResult::new(owned_sample, obj_val, cf, vf, feasible))
    }

    pub fn set_sense(&mut self, sense: Sense) -> &mut Self {
        self.sense = sense;
        self
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.environment.borrow().id == other.environment.borrow().id
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
            .field("environment_id", &self.environment.borrow().id)
            .finish()
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_model(&self).to_string();
        f.write_str(&s)
    }
}
