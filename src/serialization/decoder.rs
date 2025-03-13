use std::{cell::RefCell, rc::Rc};

use super::{
    compression::decompress,
    constraint::SerConstraints,
    environment::SerEnvironment,
    expression::SerExpression,
    model::SerModel,
    versioned::{unversionize, Version},
};
use crate::core::{Constraints, Environment, Expression, Model, VarId};
use prost::DecodeError;

pub fn decode_expression(
    data: &[u8],
    env: Rc<RefCell<Environment<VarId>>>,
) -> Result<Expression<VarId, f64>, DecodeError> {
    Ok(SerExpression::decode(decompress(data)?.as_slice())?.extract(env))
}

pub fn decode_constraints(
    data: &[u8],
    env: Rc<RefCell<Environment<VarId>>>,
) -> Result<Constraints<VarId, f64>, DecodeError> {
    Ok(SerConstraints::decode(decompress(data)?.as_slice())?.extract(env)?)
}

pub fn decode_environment(data: &[u8]) -> Result<Environment<VarId>, DecodeError> {
    Ok(SerEnvironment::decode(decompress(data)?.as_slice())?.extract())
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
