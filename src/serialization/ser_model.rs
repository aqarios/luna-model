use std::cell::RefCell;
use std::rc::Rc;

use prost::{DecodeError, Message};

use crate::core::{Constraints, Model, VarId};

use super::ser_constr::SerializableConstraints;
use super::ser_env::SerializableEnvironment;
use super::ser_expression::SerializableExpression;

#[derive(Clone, PartialEq, Message)]
pub struct SerializableModel {
    #[prost(string, tag = "1")]
    name: String,
    #[prost(message, tag = "2")]
    objective: Option<SerializableExpression>,
    #[prost(message, tag = "3")]
    constraints: Option<SerializableConstraints>,
    #[prost(message, tag = "4")]
    environment: Option<SerializableEnvironment>,
}

impl SerializableModel {
    pub fn new(model: &Model<VarId, f64>) -> Self {
        let objective = Some(SerializableExpression::new(model.objective.borrow()));

        let constraints: Option<SerializableConstraints>;
        if model.constraints.borrow().len() != 0 {
            constraints = Some(SerializableConstraints::new(model.constraints.borrow()));
        } else {
            constraints = None
        }

        Self {
            name: model.name.clone(),
            environment: Some(SerializableEnvironment::new(model.environment.borrow())),
            objective,
            constraints,
        }
    }

    pub fn decoded(data: &[u8]) -> Result<Model<VarId, f64>, DecodeError> {
        Ok(SerializableModel::decode(data)?.extract())
    }

    pub fn extract(&self) -> Model<VarId, f64> {
        let environment = Rc::new(RefCell::new(self.environment.as_ref().unwrap().extract()));
        let objective = Rc::new(RefCell::new(
            self.objective
                .as_ref()
                .unwrap()
                .extract(Rc::clone(&environment)),
        ));
        let constraints = if self.constraints.as_ref().is_some() {
            Rc::new(RefCell::new(
                self.constraints
                    .as_ref()
                    .unwrap()
                    .extract(Rc::clone(&environment)),
            ))
        } else {
            Rc::new(RefCell::new(Constraints::default()))
        };
        let mut model = Model::new(Some(self.name.clone()));
        model.objective = objective;
        model.environment = environment;
        model.constraints = constraints;
        model
    }
}
