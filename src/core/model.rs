use std::cell::RefCell;
use std::rc::Rc;

use super::environment::add_variable;
use super::expression::{
    BiasConstraints, ExpressionBaseAdd, ExpressionBaseAdjustment, ExpressionBaseCreation,
    IndexConstraints,
};
use super::{Environment, Expression, Vtype};

pub struct Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub name: String,
    pub objective: Rc<RefCell<Expression<Index, Bias>>>,
    // a model has it's own environment. This allows us to define
    // the operations more easily on the model. Getting rid of the
    // problems involving environment passing for multiplication etc.
    pub environment: Rc<RefCell<Environment<Index>>>,
    // pub constraints: Constraints,
    // pub variables: VariableStorage,
}

impl<Index, Bias> Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new_with_env(name: Option<String>, env: Rc<RefCell<Environment<Index>>>) -> Self {
        Self {
            name: name.unwrap_or(String::from("unnamed")),
            objective: Rc::new(RefCell::new(Expression::new(env.clone()))),
            environment: env,
        }
    }

    pub fn new(name: Option<String>) -> Self {
        let rcenv = Rc::new(RefCell::new(Environment::new()));
        Self {
            name: name.unwrap_or(String::from("unnamed")),
            objective: Rc::new(RefCell::new(Expression::new(rcenv.clone()))),
            environment: rcenv,
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
}
