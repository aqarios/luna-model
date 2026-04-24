//! Errors originating from pass orchestration and pipeline validation.

use std::fmt::Display;

use lunamodel_error::{ErrString, LunaModelError};

/// Errors originating from pass orchestration rather than domain modeling itself.
pub enum TransformationError {
    /// A requested analysis result was missing.
    MissingAnalysis { name: String },
    /// An analysis result existed under a key but had the wrong type.
    MismatchedAnalysis { name: String, tpe: String },
    /// A pass was scheduled before one of its requirements had been satisfied.
    UnsatisfiedRequirement {
        pass_name: String,
        requirement: String,
    },
    /// Backward execution requested a pass that was never registered.
    UnregisteredPass { name: String },
    /// An erased artifact was restored as the wrong concrete type.
    ArtifactTypeMismatch { expected: String, found: String },
}

impl Display for TransformationError {
    /// Formats the orchestration error for developers.
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
    /// Converts a transformation error into the workspace string wrapper.
    fn from(val: TransformationError) -> Self {
        val.to_string().into()
    }
}

impl From<TransformationError> for LunaModelError {
    /// Maps orchestration failures into the compilation error category.
    fn from(value: TransformationError) -> Self {
        Self::Compilation(value.into())
    }
}
