//! Errors originating from pass orchestration and pipeline validation.

use std::{error::Error, fmt::Display};

use lunamodel_error::{ErasedRecord, ErrString, LunaModelError};

use crate::{PassEntry, TransformationRecord};

#[derive(Debug)]
pub struct TranspilerError {
    pub kind: TranspileErrorKind,
    pub record: Option<TransformationRecord>,
}
impl Error for TranspilerError {}

pub type TranspilerResult<T> = Result<T, TranspilerError>;
pub type TranspileKindResult<T> = Result<T, TranspileErrorKind>;

/// Errors originating from pass orchestration rather than domain modeling itself.
#[derive(Debug)]
pub enum TranspileErrorKind {
    /// A requested analysis result was missing.
    MissingAnalysis {
        name: String,
    },
    /// An analysis result existed under a key but had the wrong type.
    MismatchedAnalysis {
        name: String,
        tpe: String,
    },
    /// A pass was scheduled before one of its requirements had been satisfied.
    UnsatisfiedRequirement {
        pass_name: String,
        requirement: String,
    },
    /// Backward execution requested a pass that was never registered.
    UnregisteredPass {
        name: String,
    },
    /// An erased artifact was restored as the wrong concrete type.
    ArtifactTypeMismatch {
        expected: String,
        found: String,
    },
    /// A query into the transformation record failed.
    RecordQuery {
        msg: &'static str,
        query: Option<String>,
    },
    /// External error occured
    External {
        e: Box<dyn Error>,
    },
    Infeasible {
        location: String,
        reason: String,
    },
}
impl Error for TranspileErrorKind {}

impl Display for TranspilerError {
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
            Self::External { e } => write!(f, "external: {}", e),
            Self::Infeasible { location, reason } => {
                write!(f, "model is infeasible at {location}: {reason}")
            }
        }
    }
}

impl TranspileErrorKind {
    pub fn external<E: Error + 'static>(e: E) -> Self {
        Self::External { e: Box::new(e) }
    }
}

impl From<LunaModelError> for TranspileErrorKind {
    fn from(value: LunaModelError) -> Self {
        Self::External { e: Box::new(value) }
    }
}

impl From<TranspileErrorKind> for LunaModelError {
    fn from(value: TranspileErrorKind) -> Self {
        match value {
            TranspileErrorKind::Infeasible { location, reason } => Self::Infeasible {
                location,
                reason,
                record: None,
            },
            _ => Self::Transformation {
                msg: value.to_string().into(),
                record: None,
            },
        }
    }
}

impl From<TranspilerError> for LunaModelError {
    fn from(value: TranspilerError) -> Self {
        let msg: ErrString = value.to_string().into();
        let TranspilerError { kind, record } = value;
        let record = record.map(ErasedRecord::new);
        match kind {
            TranspileErrorKind::Infeasible { location, reason } => LunaModelError::Infeasible {
                location,
                reason,
                record,
            },
            _ => LunaModelError::Transformation { msg, record },
        }
    }
}

impl From<TranspileErrorKind> for TranspilerError {
    fn from(kind: TranspileErrorKind) -> Self {
        Self { kind, record: None }
    }
}

impl From<LunaModelError> for TranspilerError {
    fn from(value: LunaModelError) -> Self {
        TranspilerError {
            kind: value.into(),
            record: None,
        }
    }
}

pub fn record(
    f: impl FnOnce(&mut Vec<PassEntry>) -> TranspilerResult<()>,
) -> TranspilerResult<TransformationRecord> {
    let mut entries = Vec::new();
    let res = f(&mut entries);
    let record = TransformationRecord { entries };
    match res {
        Ok(_) => Ok(record),
        Err(TranspilerError { kind, .. }) => Err(TranspilerError {
            kind,
            record: Some(record),
        }),
    }
}

/// Pushes the nested entry (with its partial record on failure) and propagates any error.
pub fn attach_nested(
    entries: &mut Vec<PassEntry>,
    res: TranspilerResult<TransformationRecord>,
    make_entry: impl FnOnce(TransformationRecord) -> PassEntry,
) -> TranspilerResult<()> {
    match res {
        Ok(record) => {
            entries.push(make_entry(record));
            Ok(())
        }
        Err(TranspilerError { kind, record }) => {
            entries.push(make_entry(record.unwrap_or_else(|| Vec::new().into())));
            Err(kind.into())
        }
    }
}
