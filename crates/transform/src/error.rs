use std::{error::Error, fmt::Display};

use lunamodel_error::LunaModelError;
use lunamodel_transpiler::{TranspileErrorKind, TranspilerError};

pub type TransformResult<T> = Result<T, TransformError>;

#[derive(Debug)]
pub enum TransformError {
    Analysis {
        name: String,
        msg: String,
    },
    Transformation {
        name: String,
        msg: String,
    },
    /// External error occured
    External {
        e: Box<dyn Error>,
    },
}
impl Error for TransformError {}

impl Display for TransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analysis { name, msg } => write!(f, "analysis '{name}' errored: {msg}"),
            Self::Transformation { name, msg } => {
                write!(f, "transformation '{name}' errored: {msg}")
            }
            Self::External { e } => write!(f, "external: {}", e.to_string()),
        }
    }
}

impl From<TransformError> for TranspileErrorKind {
    fn from(value: TransformError) -> Self {
        TranspileErrorKind::External { e: Box::new(value) }
    }
}

impl From<LunaModelError> for TransformError {
    fn from(value: LunaModelError) -> Self {
        Self::External { e: Box::new(value) }
    }
}

impl From<TranspilerError> for TransformError {
    fn from(value: TranspilerError) -> Self {
        Self::External { e: Box::new(value) }
    }
}
