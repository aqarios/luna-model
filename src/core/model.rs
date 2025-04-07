use super::constraints::Constraints;
use super::environment::add_variable;
use super::expression::{
    BiasConstraints, ExpressionBaseAdd, ExpressionBaseAdjustment, ExpressionBaseCreation,
    IndexConstraints,
};
use super::{Environment, Expression, IndexByValue, RcSolution, Vtype};
use crate::core::solution::{AssignmentBaseTypes, OwnedResult};
use crate::core::utils::ModelWriter;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Mul;
use std::rc::Rc;

/// The default name for a model.
static DEFAULT_MODEL_NAME: &str = "unnamed";

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
        }
    }

    /// Create a new Model based on a Quadratic Unconstrained Binary Optimization (QUBO)
    /// problem represented as a continuous slice of all values. A new environment is
    /// created based on the size of the QUBO.
    pub fn new_from_dense(
        name: Option<String>,
        dense: &[Bias],
        num_variables: Index,
        vtype: Vtype,
    ) -> Self {
        let model = Model::new(name);
        // We also need to add the varaibles to the model...
        (0..num_variables.into()).into_iter().for_each(|idx| {
            let _ = add_variable(
                model.environment.clone(),
                &idx.to_string(),
                Some(&vtype),
                None,
            );
        });

        model.objective.borrow_mut().resize(num_variables);
        model
            .objective
            .borrow_mut()
            .add_quadratic_from_dense(dense, num_variables);
        model
    }

    fn evaluate_solution<AssignmentTypes>(
        &self,
        _sol: RcSolution<Bias, AssignmentTypes>,
    ) -> RcSolution<Bias, AssignmentTypes>
    where
        AssignmentTypes: AssignmentBaseTypes,
    {
        // Here, duplicate samples are already removed, i.e., each element of sol.samples is unique

        todo!("Implement evaluation logic")
    }

    fn evaluate_sample<'a, AssignmentTypes, Elem: 'a, Sample: IndexByValue<Index, Output = Elem>>(
        &self,
        _sample: &'a Sample,
    ) -> OwnedResult<Bias, AssignmentTypes>
    where
        AssignmentTypes: AssignmentBaseTypes,
        &'a Elem: Mul<Bias, Output = Bias>,
        Elem: Mul<Bias, Output = Bias>,
    {
        todo!("Implement evaluation logic")
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
