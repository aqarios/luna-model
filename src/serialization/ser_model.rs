use std::cell::RefCell;
use std::rc::Rc;

use prost::{DecodeError, Message};

use crate::core::{Model, VarId};

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
    environment: Option<SerializableEnvironment>,
    #[prost(message, tag = "4")]
    constraints: Option<SerializableConstraints>,
}

impl SerializableModel {
    pub fn new(model: &Model<VarId, f64>) -> Self {
        Self {
            name: model.name.clone(),
            objective: Some(SerializableExpression::new(model.objective.borrow())),
            environment: Some(SerializableEnvironment::new(model.environment.borrow())),
            constraints: Some(SerializableConstraints::new(model.constraints.borrow())),
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
        let constraints = Rc::new(RefCell::new(
            self.constraints
                .as_ref()
                .unwrap()
                .extract(Rc::clone(&environment)),
        ));
        let mut model = Model::new(Some(self.name.clone()));
        model.objective = objective;
        model.environment = environment;
        model.constraints = constraints;
        model
    }
}
