use super::{
    compression::{compress, decompress},
    constraint::{decode_constraints, encode_constraints},
    decode_expression, encode_expression,
    environment::{decode_environment, encode_environment},
    versioned::{unversionize, versionize, Version},
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
    fn empty(name: String) -> Self {
        Self {
            objective: Vec::new(),
            constraints: Vec::new(),
            environment: Vec::new(),
            name,
        }
    }

    fn new(
        model: &Model<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        Self::empty(model.name.clone()).fill(&model, use_compression, level)
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

    fn extract(&self) -> Result<Model<VarId, f64>, DecodeError> {
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

pub fn encode_model_v0(
    model: &Model<VarId, f64>,
    use_compression: bool,
    level: Option<i32>,
) -> Result<Vec<u8>, io::Error> {
    Ok(versionize(
        compress(
            SerModel::new(model, use_compression, level)?.encode_to_vec(),
            use_compression,
            level,
        )?,
        Version::V0,
    ))
}

/// Alias for the latest version.
pub fn encode_model(
    model: &Model<VarId, f64>,
    use_compression: bool,
    level: Option<i32>,
) -> Result<Vec<u8>, io::Error> {
    encode_model_v0(model, use_compression, level)
}

pub fn decode_model_v0(data: &[u8]) -> Result<Model<VarId, f64>, DecodeError> {
    SerModel::decode(decompress(data)?.as_slice())?.extract()
}

pub fn decode_model(data: &[u8]) -> Result<Model<VarId, f64>, DecodeError> {
    let versioned = unversionize(data)?;
    if versioned.version == Version::V0 as u32 {
        decode_model_v0(versioned.data.as_slice())
    } else {
        Err(DecodeError::new(format!(
            "unknown version: {}",
            versioned.version
        )))
    }
}
