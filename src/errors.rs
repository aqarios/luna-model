use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug, Clone)]
pub struct VariableExistsErr;
impl Error for VariableExistsErr {}
impl Display for VariableExistsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "variable already exists in environment")
    }
}

#[derive(Debug, Clone)]
pub struct VariableCreationErr {
    msg: String,
}
impl VariableCreationErr {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Error for VariableCreationErr {}
impl Display for VariableCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "variable creation failed: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct VariablesFromDifferentEnvsErr;
impl Error for VariablesFromDifferentEnvsErr {}
impl Display for VariablesFromDifferentEnvsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct DifferentEnvsErr;
impl Error for DifferentEnvsErr {}
impl Display for DifferentEnvsErr {
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
pub struct ModelNotQuadraticErr;
impl Error for ModelNotQuadraticErr {}
impl Display for ModelNotQuadraticErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "the model is not linear or quadratic, thus cannot be translated to a matrix."
        )
    }
}

#[derive(Debug, Clone)]
pub struct ModelNotUnconstrainedErr;
impl Error for ModelNotUnconstrainedErr {}
impl Display for ModelNotUnconstrainedErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the model is not unconstrained")
    }
}

#[derive(Debug, Clone)]
pub enum MatrixTranslatorErr {
    Constrained(ModelNotUnconstrainedErr),
    HigherOrder(ModelNotQuadraticErr),
}
impl Error for MatrixTranslatorErr {}
impl Display for MatrixTranslatorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            MatrixTranslatorErr::Constrained(err) => err.fmt(f),
            MatrixTranslatorErr::HigherOrder(err) => err.fmt(f),
        }
    }
}

impl From<ModelNotQuadraticErr> for MatrixTranslatorErr {
    fn from(value: ModelNotQuadraticErr) -> Self {
        Self::HigherOrder(value)
    }
}
impl From<ModelNotUnconstrainedErr> for MatrixTranslatorErr {
    fn from(value: ModelNotUnconstrainedErr) -> Self {
        Self::Constrained(value)
    }
}

#[derive(Debug, Clone)]
pub struct IndexOutOfBoundsErr {
    idx: usize,
    len: usize,
}
impl IndexOutOfBoundsErr {
    pub fn new(idx: usize, len: usize) -> Self {
        Self { idx, len }
    }
}
impl Error for IndexOutOfBoundsErr {}
impl Display for IndexOutOfBoundsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "index '{}' out of bounds for constraints of len {}",
            self.idx, self.len
        )
    }
}
