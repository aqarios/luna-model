use std::fmt::{Display, Result};

use crate::errors::VariablesFromDifferentEnvsErr;

// VariableOutOfRangeError
#[derive(Debug, Clone)]
pub struct VariableOutOfRangeErr(pub usize);

impl std::error::Error for VariableOutOfRangeErr {}

impl Display for VariableOutOfRangeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "variable is out of range (is {})", self.0)
    }
}

// IndexOutOfOrderError
#[derive(Debug, Clone)]
pub struct IndexOutOfOrderError(pub usize, pub usize);

impl std::error::Error for IndexOutOfOrderError {}

impl Display for IndexOutOfOrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Index out of oder: last index <= {} (is {})",
            self.0, self.1
        )
    }
}

// VariableOutOfRangeError
#[derive(Debug, Clone)]
pub enum VariableError {
    VariableFromDifferentEnvs(VariablesFromDifferentEnvsErr),
    VariableOutOfRangeError(VariableOutOfRangeErr),
}

impl std::error::Error for VariableError {}

impl Display for VariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::VariableOutOfRangeError(e) => e.fmt(f),
            Self::VariableFromDifferentEnvs(e) => e.fmt(f),
        }
    }
}

impl Into<VariableError> for VariablesFromDifferentEnvsErr {
    fn into(self) -> VariableError {
        VariableError::VariableFromDifferentEnvs(self)
    }
}

impl Into<VariableError> for VariableOutOfRangeErr {
    fn into(self) -> VariableError {
        VariableError::VariableOutOfRangeError(self)
    }
}

#[derive(Debug, Clone)]
pub struct EnvMismatchError;

impl std::error::Error for EnvMismatchError {}

impl Display for EnvMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Environments of the provided models do not match.",)
    }
}
