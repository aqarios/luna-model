use super::constraints::Constraints;
use super::environment::add_variable;
use super::expression::{
    AssignmentConstraints, BiasConstraints, ExpressionBaseAdd, ExpressionBaseAdjustment,
    ExpressionBaseCreation, IndexConstraints,
};
use super::{Environment, Expression, Vtype};
use crate::core::solution::Solution;
use crate::core::utils::ModelWriter;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

static DEFAULT_MODEL_NAME: &str = "unnamed";

pub struct Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub name: String,
    pub objective: Rc<RefCell<Expression<Index, Bias>>>,
    // a model has its own environment. This allows us to define
    // the operations more easily on the model. Getting rid of the
    // problems involving environment passing for multiplication etc.
    pub environment: Rc<RefCell<Environment<Index>>>,
    pub constraints: Rc<RefCell<Constraints<Index, Bias>>>,
}

impl<Index, Bias> Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new_with_env(name: Option<String>, env: Rc<RefCell<Environment<Index>>>) -> Self {
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Rc::new(RefCell::new(Expression::empty(env.clone()))),
            environment: env,
            constraints: Rc::new(RefCell::new(Constraints::default())),
        }
    }

    pub fn new(name: Option<String>) -> Self {
        let rcenv = Rc::new(RefCell::new(Environment::new()));
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Rc::new(RefCell::new(Expression::empty(rcenv.clone()))),
            environment: rcenv,
            constraints: Rc::new(RefCell::new(Constraints::default())),
        }
    }

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

    fn evaluate<Assign>(&self, _sol: &mut Solution<Assign, Bias>) -> &mut Solution<Assign, Bias>
    where
        Assign: AssignmentConstraints,
    {
        // Here, duplicate samples are already removed, i.e., each element of sol.samples is unique

        todo!("Implement evaluation logic")
    }
}

impl<Index, Bias> PartialEq for Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn eq(&self, other: &Self) -> bool {
        // Add contraints
        // Remove name and environment
        self.name == other.name
            && self.environment.borrow().id == other.environment.borrow().id
            && self.objective == other.objective
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
