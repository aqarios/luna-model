//! Errors originating from pass orchestration and pipeline validation.

use std::{error::Error, fmt::Display};

use crate::{PassEntry, TransformationRecord};

#[derive(Debug)]
pub struct TranspileError {
    pub kind: TranspileErrorKind,
    pub record: Option<TransformationRecord>,
}
impl Error for TranspileError {}

pub type TranspileResult<T> = Result<T, TranspileError>;
pub type TranspileKindResult<T> = Result<T, TranspileErrorKind>;

/// Errors originating from pass orchestration rather than domain modeling itself.
#[derive(Debug)]
pub enum TranspileErrorKind {
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
    /// A query into the transformation record failed.
    RecordQuery {
        msg: &'static str,
        query: Option<String>,
    },
    /// External error occured
    External { e: Box<dyn Error> },
}
impl Error for TranspileErrorKind {}

impl Display for TranspileError {
    /// Formats the orchestration error for developers.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for TranspileErrorKind {
    /// Formats the orchestration error for developers.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAnalysis { name } => {
                write!(f, "missing analysis pass '{name}'")
            }
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
            Self::RecordQuery { msg, query } => match query {
                Some(q) => write!(f, "query failed: {msg} for '{q}'"),
                None => write!(f, "query failed: {msg}"),
            },
            Self::External { e } => write!(f, "external: {}", e.to_string()),
        }
    }
}

impl From<TranspileErrorKind> for TranspileError {
    fn from(kind: TranspileErrorKind) -> Self {
        Self { kind, record: None }
    }
}

pub fn record(
    f: impl FnOnce(&mut Vec<PassEntry>) -> TranspileResult<()>,
) -> TranspileResult<TransformationRecord> {
    let mut entries = Vec::new();
    let res = f(&mut entries);
    let record = TransformationRecord { entries };
    match res {
        Ok(_) => Ok(record),
        Err(TranspileError { kind, .. }) => Err(TranspileError {
            kind,
            record: Some(record),
        }),
    }
}

impl<E: Error + 'static> From<E> for TransformError {
    fn from(value: E) -> Self {
        Self::External { e: Box::new(value) }
    }
}
