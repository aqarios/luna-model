use super::{
    decoder::{decode_constraints, decode_environment, decode_expression},
    encoder::{encode_constraints, encode_environment, encode_expression},
};
use crate::core::{Model, VarId};
use prost::{DecodeError, Message};
use std::{cell::RefCell, io, rc::Rc};

#[derive(Clone, PartialEq, Message)]
pub struct SerModel {
    /// The model's objective serialized and maybe compressed
    #[prost(bytes, tag = "1")]
    objective: Vec<u8>,
    /// The model's constraints serialized and maybe compressed
    #[prost(bytes, tag = "2")]
    constraints: Vec<u8>,
    /// The model's environment serialized and maybe compressed
    #[prost(bytes, tag = "3")]
    environment: Vec<u8>,
    /// The model's name
    #[prost(string, tag = "4")]
    name: String,
}

impl SerModel {
    pub fn new(
        model: &Model<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        Self::empty(model.name.clone()).fill(&model, use_compression, level)
    }

    fn empty(name: String) -> Self {
        Self {
            objective: Vec::new(),
            constraints: Vec::new(),
            environment: Vec::new(),
            name,
        }
    }

    fn fill(
        mut self,
        model: &Model<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        self.objective = encode_expression(&model.objective.borrow(), use_compression, level)?;
        self.constraints = encode_constraints(&model.constraints.borrow(), use_compression, level)?;
        self.environment = encode_environment(&model.environment.borrow(), use_compression, level)?;
        Ok(self)
    }

    pub fn extract(&self) -> Result<Model<VarId, f64>, DecodeError> {
        let mut model = Model::new(Some(self.name.clone()));
        model.environment = Rc::new(RefCell::new(decode_environment(
            self.environment.as_slice(),
        )?));
        model.objective = Rc::new(RefCell::new(decode_expression(
            self.objective.as_slice(),
            Rc::clone(&model.environment),
        )?));
        model.constraints = Rc::new(RefCell::new(decode_constraints(
            self.constraints.as_slice(),
            Rc::clone(&model.environment),
        )?));
        Ok(model)
    }
}
