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
    Backward {
        id: String,
        msg: String,
    },
    /// External error occured
    External {
        e: anyhow::Error,
    },
    /// Model is infeasible.
    Infeasible {
        location: String,
        reason: String,
    },
}
impl Error for TransformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::External { e } => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl Display for TransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analysis { name, msg } => write!(f, "analysis '{name}' failed: {msg}"),
            Self::Transformation { name, msg } => {
                write!(f, "transformation '{name}' failed: {msg}")
            }
            Self::Backward { id, msg } => {
                write!(f, "backward '{id}' failed: {msg}")
            }
            Self::External { e } => write!(
                f,
                "external: {}",
                e.chain()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" - caused by: ")
            ),
            Self::Infeasible { location, reason } => {
                write!(f, "model is infeasible at {location}: {reason}")
            }
        }
    }
}

impl From<TransformError> for TranspileErrorKind {
    fn from(value: TransformError) -> Self {
        match value {
            TransformError::Infeasible { location, reason } => {
                TranspileErrorKind::Infeasible { location, reason }
            }
            _ => TranspileErrorKind::External {
                e: anyhow::Error::from(value),
            },
        }
    }
}

impl From<LunaModelError> for TransformError {
    fn from(value: LunaModelError) -> Self {
        Self::External {
            e: anyhow::Error::from(value),
        }
    }
}

impl From<TranspilerError> for TransformError {
    fn from(value: TranspilerError) -> Self {
        Self::External {
            e: anyhow::Error::from(value),
        }
    }
}
