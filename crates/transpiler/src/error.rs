use std::fmt::Display;

use lunamodel_error::{ErrString, LunaModelError};

// TODO(team): validation error should be part of TransformationError not how it's currently done.
pub struct ValidationError(pub TransformationError);

pub enum TransformationError {
    MissingAnalysis {
        name: String,
    },
    MismatchedAnalysis {
        name: String,
        tpe: String,
    },
    UnsatisfiedRequirement {
        pass_name: String,
        requirement: String,
    },
    UnregisteredPass {
        name: String,
    },
    ArtifactTypeMismatch {
        expected: String,
        found: String,
    },
}

impl Display for TransformationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAnalysis { name } => write!(f, "missing analysis pass '{name}'"),
            Self::MismatchedAnalysis { name, tpe } => {
                write!(f, "mismatched analysis pass type '{tpe}' for key '{name}'")
            }
            Self::UnsatisfiedRequirement {
                pass_name,
                requirement,
            } => {
                write!(
                    f,
                    "pass '{pass_name}' requires '{requirement}' to be satisfied first"
                )
            }
            Self::UnregisteredPass { name } => {
                write!(f, "unregistered pass for backwards '{name}'")
            }
            Self::ArtifactTypeMismatch { expected, found } => {
                write!(
                    f,
                    "artifact type mismatch: expected '{expected}', found '{found}'"
                )
            }
        }
    }
}

impl From<TransformationError> for ErrString {
    fn from(val: TransformationError) -> Self {
        val.to_string().into()
    }
}

impl From<TransformationError> for LunaModelError {
    fn from(value: TransformationError) -> Self {
        Self::Compilation(value.into())
    }
}

impl From<ValidationError> for LunaModelError {
    fn from(value: ValidationError) -> Self {
        Self::Compilation(format!("validation failed: {}", value.0).into())
    }
}
