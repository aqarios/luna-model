use std::fmt::{Display, Result};

use crate::core::exceptions::VariablesFromDifferentEnvsError;

// VariableOutOfRangeError
#[derive(Debug, Clone)]
pub struct VariableOutOfRangeError(pub usize);

impl std::error::Error for VariableOutOfRangeError {}

impl Display for VariableOutOfRangeError {
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
    VariableFromDifferentEnvs(VariablesFromDifferentEnvsError),
    VariableOutOfRangeError(VariableOutOfRangeError),
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

impl Into<VariableError> for VariablesFromDifferentEnvsError {
    fn into(self) -> VariableError {
        VariableError::VariableFromDifferentEnvs(self)
    }
}

impl Into<VariableError> for VariableOutOfRangeError {
    fn into(self) -> VariableError {
        VariableError::VariableOutOfRangeError(self)
    }
}
