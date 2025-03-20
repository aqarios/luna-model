use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug, Clone)]
pub struct VariableExistsError;
impl Error for VariableExistsError {}
impl Display for VariableExistsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "variable already exists in environment")
    }
}

#[derive(Debug, Clone)]
pub struct VariableCreationError {
    msg: String,
}
impl VariableCreationError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Error for VariableCreationError {}
impl Display for VariableCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "variable creation failed: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct VariablesFromDifferentEnvsError;
impl Error for VariablesFromDifferentEnvsError {}
impl Display for VariablesFromDifferentEnvsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct DifferentEnvsError;
impl Error for DifferentEnvsError {}
impl Display for DifferentEnvsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ParseFromStringError(pub String);
impl Error for ParseFromStringError {}
impl Display for ParseFromStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "could not parse to string: {}", self.0)
    }
}
impl From<ParseIntError> for ParseFromStringError {
    fn from(err: ParseIntError) -> Self {
        ParseFromStringError(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ModelNotQuadraticError;
impl Error for ModelNotQuadraticError {}
impl Display for ModelNotQuadraticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "the model is not linear or quadratic, thus cannot be translated to a matrix."
        )
    }
}

#[derive(Debug, Clone)]
pub struct ModelNotUnconstrainedError;
impl Error for ModelNotUnconstrainedError {}
impl Display for ModelNotUnconstrainedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the model is not unconstrained")
    }
}

#[derive(Debug, Clone)]
pub enum MatrixTranslatorError {
    Constrained(ModelNotUnconstrainedError),
    HigherOrder(ModelNotQuadraticError),
}
impl Error for MatrixTranslatorError {}
impl Display for MatrixTranslatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            MatrixTranslatorError::Constrained(err) => err.fmt(f),
            MatrixTranslatorError::HigherOrder(err) => err.fmt(f),
        }
    }
}

impl From<ModelNotQuadraticError> for MatrixTranslatorError {
    fn from(value: ModelNotQuadraticError) -> Self {
        Self::HigherOrder(value)
    }
}
impl From<ModelNotUnconstrainedError> for MatrixTranslatorError {
    fn from(value: ModelNotUnconstrainedError) -> Self {
        Self::Constrained(value)
    }
}
