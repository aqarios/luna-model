use strum_macros::Display;

use super::constraints::Constraints;
use super::environment::add_variable;
use super::expression::{
    BiasConstraints, ExpressionBaseAdd, ExpressionBaseAdjustment, ExpressionBaseCreation,
    IndexConstraints,
};
use super::{Environment, Expression, RcSolution, Sample, Vtype};
use crate::core::expression::ExpressionEvaluation;
use crate::core::solution::{AssignmentBaseTypes, OwnedResult};
use crate::core::writer::ModelWriter;
use crate::errors::TranslationErr;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[cfg(feature = "py")]
use pyo3::prelude::*;

/// The default name for a model.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

#[cfg_attr(
    feature = "py",
    pyclass(eq, eq_int, name = "Sense", module = "aqmodels")
)] // we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
#[derive(Display, Copy, PartialEq, Hash, Clone, Debug, Eq)]
/// The optimization sense, i.e., the direction to be optimized towards.
pub enum Sense {
    #[strum(to_string = "Minimize")]
    Min,
    #[strum(to_string = "Maximize")]
    Max,
}

impl Sense {
    /// Convenience function to check if the sense is `Sense::Min`.
    pub fn is_min(&self) -> bool {
        self == &Self::Min
    }
}

/// A model describing some function to be optimized (objective) and restrictions
/// on this objective (constraints).
pub struct Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// The name of the model.
    pub name: String,
    /// The environment of the model, constaining the information for each variable
    /// used in both the objective and it's constraints.
    pub environment: Rc<RefCell<Environment<Index>>>,
    /// The objective of the model describing some optimization problem. The objective
    /// is an expression that can be linear, quadratic or higher order.
    pub objective: Rc<RefCell<Expression<Index, Bias>>>,
    /// The constraints of the model describing the restrictions on the model.
    pub constraints: Rc<RefCell<Constraints<Index, Bias>>>,
    /// The sense of the model, i.e., the direction to be optimized at.
    /// By default is set to `Sense::Min`.
    pub sense: Sense,
}

impl<Index, Bias> Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Create a new Model using a specifc environment.
    pub fn new_with_env(name: Option<String>, env: Rc<RefCell<Environment<Index>>>) -> Self {
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Rc::new(RefCell::new(Expression::empty(env.clone()))),
            environment: env,
            constraints: Rc::new(RefCell::new(Constraints::default())),
            sense: Sense::Min,
        }
    }

    /// Create a new Model with a new environment created just for this model.
    pub fn new(name: Option<String>) -> Self {
        let rcenv = Rc::new(RefCell::new(Environment::new()));
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Rc::new(RefCell::new(Expression::empty(rcenv.clone()))),
            environment: rcenv,
            constraints: Rc::new(RefCell::new(Constraints::default())),
            sense: Sense::Min,
        }
    }

    /// Create a new Model based on a Quadratic Unconstrained Binary Optimization (QUBO)
    /// problem represented as a continuous slice of all values. A new environment is
    /// created based on the size of the QUBO.
    pub fn new_from_dense(
        name: Option<String>,
        vtype: Option<Vtype>,
        matrix_flat: &[Bias],
        num_variables: Index,
        offset: Option<Bias>,
        variable_names: Option<Vec<String>>,
    ) -> Result<Self, TranslationErr> {
        let model = Model::new(name);
        // We also need to add the variables to the model...
        for idx in 0..num_variables.into() {
            let var_name = match &variable_names {
                None => &format!("x_{}", idx.to_string()),
                Some(names) => {
                    // Name needs to start with alpha.
                    let name = &names[idx];
                    if !name.starts_with(|c: char| c.is_alphabetic()) {
                        return Err(TranslationErr::new(String::from(
                            "Variable names must start with an alphabetic character.",
                        )));
                    }
                    for c in name.chars() {
                        // Check that the character is only alphanumeric or '_' or ','.
                        if c.is_alphanumeric() || c == '_' || c == ',' {
                            continue;
                        } else {
                            return Err(TranslationErr::new(String::from(
                                "Variable names must only contain alphanumeric characters or '_' or ','."
                            )));
                        }
                    }
                    name
                }
            };
            let _ = add_variable(
                model.environment.clone(),
                var_name,
                Some(&vtype.unwrap_or(Vtype::Binary)),
                None,
            );
        }

        model.objective.borrow_mut().resize(num_variables);
        model
            .objective
            .borrow_mut()
            .add_quadratic_from_dense(matrix_flat, num_variables);
        if let Some(off) = offset {
            model.objective.borrow_mut().add_offset(off);
        }
        Ok(model)
    }

    pub fn evaluate_solution<AssignmentTypes>(
        &self,
        sol: RcSolution<Bias, AssignmentTypes>,
    ) -> RcSolution<Bias, AssignmentTypes>
    where
        AssignmentTypes: AssignmentBaseTypes,
    {
        let mut newsol = sol.0.deref().clone();
        for (i, sample) in sol.samples().iter().enumerate() {
            let obj_val = self.objective.borrow().evaluate_sample(&sample);
            let constraints = if self.constraints.borrow().is_empty() {
                None
            } else {
                Some(
                    self.constraints
                        .borrow()
                        .iter()
                        .map(|constr| constr.evaluate_sample(&sample))
                        .collect(),
                )
            };
            newsol.add_sample_evaluation(i, Some(obj_val), constraints, self.sense.is_min());
        }
        RcSolution(newsol.into())
    }

    pub fn evaluate_sample<'a, AssignmentTypes>(
        &self,
        sample: &Sample<Bias, AssignmentTypes>,
    ) -> OwnedResult<Bias, AssignmentTypes>
    where
        AssignmentTypes: AssignmentBaseTypes,
    {
        let obj_val = self.objective.borrow().evaluate_sample(sample);
        let cf: Vec<_> = self
            .constraints
            .borrow()
            .iter()
            .map(|constraint| {
                let v = constraint.lhs.borrow().evaluate_sample(sample);
                constraint.comparator.evaluate(v, constraint.rhs)
            })
            .collect();
        let feasible = cf.iter().all(|&b| b);
        let owned_sample = Rc::new(sample.iter().collect());
        OwnedResult::new(owned_sample, obj_val, cf, feasible)
    }

    pub fn set_sense(&mut self, sense: Sense) -> &mut Self {
        self.sense = sense;
        self
    }
}

impl<Index, Bias> PartialEq for Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.environment.borrow().id == other.environment.borrow().id
            && self.objective == other.objective
            && self.constraints == other.constraints
    }
}

impl<Index, Bias> Debug for Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("name", &self.name)
            .field("objective", &self.objective.borrow())
            .field("constraints", &self.constraints.borrow())
            .field("environment_id", &self.environment.borrow().id)
            .finish()
    }
}

impl<Index, Bias> Display for Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_model(&self).to_string();
        f.write_str(&s)
    }
}
