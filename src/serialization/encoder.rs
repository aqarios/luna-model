use super::{
    compression::compress,
    constraint::SerConstraints,
    environment::SerEnvironment,
    expression::SerExpression,
    model::SerModel,
    versioned::{versionize, Version},
};
use crate::core::{Constraints, Environment, Expression, Model, VarId};
use prost::Message;
use std::io;

pub fn encode_expression(
    expression: &Expression<VarId, f64>,
    use_compression: bool,
    level: Option<i32>,
) -> Result<Vec<u8>, std::io::Error> {
    compress(
        SerExpression::new(expression).encode_to_vec(),
        use_compression,
        level,
    )
}

pub fn encode_constraints(
    constraints: &Constraints<VarId, f64>,
    use_compression: bool,
    level: Option<i32>,
) -> Result<Vec<u8>, io::Error> {
    compress(
        SerConstraints::new(constraints, use_compression, level)?.encode_to_vec(),
        use_compression,
        level,
    )
}

pub fn encode_environment(
    environment: &Environment<VarId>,
    use_compression: bool,
    level: Option<i32>,
) -> Result<Vec<u8>, io::Error> {
    compress(
        SerEnvironment::new(environment).encode_to_vec(),
        use_compression,
        level,
    )
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
