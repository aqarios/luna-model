//! Errors originating from pass orchestration and pipeline validation.

use std::{error::Error, fmt::Display, sync::Arc};

use lunamodel_error::{ErasedRecord, ErrString, LunaModelError};

use crate::{PassEntry, TransformationRecord};

#[derive(Debug)]
pub struct TranspilerError {
    pub kind: TranspileErrorKind,
    pub record: Option<TransformationRecord>,
}
impl Error for TranspilerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

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
        e: anyhow::Error,
        /// The originating kind tag, when known (e.g. a plugin's own `RichError.kind`), so it
        /// survives into the resulting [`LunaModelError`]'s `kind_tag()` instead of collapsing
        /// to the generic `"transformation"`/`"external"` tag.
        kind: Option<String>,
    },
    Infeasible {
        location: String,
        reason: String,
    },
}
impl Error for TranspileErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::External { e, .. } => Some(e.as_ref()),
            _ => None,
        }
    }
}

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
            Self::External { e, .. } => write!(f, "external: {}", describe_external(e)),
            Self::Infeasible { location, reason } => {
                write!(f, "model is infeasible at {location}: {reason}")
            }
        }
    }
}

impl TranspileErrorKind {
    pub fn external<E: Error + Send + Sync + 'static>(e: E) -> Self {
        Self::External {
            e: anyhow::Error::from(e),
            kind: None,
        }
    }

    /// Like [`external`](Self::external), but tags the resulting error with `kind` (e.g. a
    /// plugin's own `RichError.kind`) so it survives into the final [`LunaModelError`]'s
    /// `kind_tag()` instead of collapsing to a generic tag.
    pub fn external_with_kind<E: Error + Send + Sync + 'static>(
        kind: impl Into<String>,
        e: E,
    ) -> Self {
        Self::External {
            e: anyhow::Error::from(e),
            kind: Some(kind.into()),
        }
    }
}

impl From<LunaModelError> for TranspileErrorKind {
    fn from(value: LunaModelError) -> Self {
        Self::External {
            e: anyhow::Error::from(value),
            kind: None,
        }
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
            TranspileErrorKind::External { e, kind } => {
                // `with_cause` attaches the rest of `e`'s chain structurally, so the top-level
                // message here is just the outermost context frame, not the whole chain
                // (avoids printing the chain twice via `WithCause`'s own `Display`).
                let msg = e.to_string().into();
                let kind = kind.map(Into::into);
                with_cause(
                    Self::Transformation {
                        msg,
                        record: None,
                        kind,
                    },
                    &e,
                )
            }
            other => Self::Transformation {
                msg: other.to_string().into(),
                record: None,
                kind: None,
            },
        }
    }
}

impl From<TranspilerError> for LunaModelError {
    fn from(value: TranspilerError) -> Self {
        let TranspilerError { kind, record } = value;
        let record = record.map(ErasedRecord::new);
        match kind {
            TranspileErrorKind::Infeasible { location, reason } => LunaModelError::Infeasible {
                location,
                reason,
                record,
            },
            TranspileErrorKind::External { e, kind } => {
                let msg = e.to_string().into();
                let kind = kind.map(Into::into);
                with_cause(LunaModelError::Transformation { msg, record, kind }, &e)
            }
            other => LunaModelError::Transformation {
                msg: other.to_string().into(),
                record,
                kind: None,
            },
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

fn describe_external(e: &anyhow::Error) -> ErrString {
    e.chain()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" - caused by: ")
        .into()
}

/// Wraps `inner` around the chain of `e` (every `anyhow::Context` frame, in order) as nested
/// [`LunaModelError::WithCause`] links, so the chain survives structurally — each link
/// individually inspectable and `Display`-able — rather than being collapsed into one flat
/// string (which [`describe_external`] still produces, for the outer message text).
///
/// Stops as soon as it reaches an embedded `LunaModelError::WithCause` (e.g. one built by
/// `map_pyerr` around a Python exception) and adopts *that* node's own cause verbatim instead of
/// re-wrapping it: `WithCause`'s cause is `Arc<dyn Error + Send + Sync>`, not itself a
/// `LunaModelError`, and boundaries like the Python bindings' `__cause__` recovery
/// (`crates/error/src/py.rs`) downcast it directly to recover the original typed error (a
/// `PyErr`, say) by object identity — re-wrapping it inside another `LunaModelError` would bury
/// it somewhere `Error::source()` can no longer reach (`source()` only walks a `WithCause`'s
/// `cause` field, never its boxed content), silently breaking that recovery. Anything further
/// down `e`'s chain past that point is just the adopted cause's own source and is already
/// reachable through it, so it needs no separate representation here.
fn with_cause(inner: LunaModelError, e: &anyhow::Error) -> LunaModelError {
    // Walk root to leaf, collecting one frame per link; stop early (see doc comment above) at an
    // embedded `WithCause`, taking its own cause as the terminal one.
    //
    // `chain()` yields `e` itself as its first element, and that top frame's *text* is already
    // captured verbatim in `inner`'s message (built from `e.to_string()` by the caller) — so it's
    // never re-pushed as its own redundant frame. The one exception is when that first link is
    // itself an embedded `WithCause`: its `cause` field (the `Arc<dyn Error + Send + Sync>`, e.g.
    // a `PyErr` from `map_pyerr`) is only reachable by matching the link itself, not by anything
    // in the rendered text, so it must still be detected here even though its content isn't
    // pushed as a duplicate frame — dropping this case entirely would silently discard the
    // terminal cause instead of merely avoiding a duplicate.
    let mut frames: Vec<LunaModelError> = Vec::new();
    let mut terminal_cause: Option<Arc<dyn Error + Send + Sync>> = None;
    let mut chain = e.chain();
    let first_is_terminal = match chain.next() {
        Some(link) => match link.downcast_ref::<LunaModelError>() {
            Some(LunaModelError::WithCause(content, cause)) => {
                frames.push((**content).clone());
                terminal_cause = Some(Arc::clone(cause));
                true
            }
            _ => false,
        },
        None => false,
    };
    if !first_is_terminal {
        for link in chain {
            match link.downcast_ref::<LunaModelError>() {
                Some(LunaModelError::WithCause(content, cause)) => {
                    frames.push((**content).clone());
                    terminal_cause = Some(Arc::clone(cause));
                    break;
                }
                Some(other) => frames.push(other.clone()),
                None => frames.push(LunaModelError::External(link.to_string().into())),
            }
        }
    }
    // Fold leaf to root: the terminal cause (if any) attaches only to the innermost (last)
    // frame; every frame further out just nests the previous, already-built result.
    let mut chained: Option<LunaModelError> = None;
    for frame in frames.into_iter().rev() {
        chained = Some(match chained {
            Some(prev) => LunaModelError::WithCause(Box::new(frame), Arc::new(prev)),
            None => match terminal_cause.take() {
                Some(cause) => LunaModelError::WithCause(Box::new(frame), cause),
                None => frame,
            },
        });
    }
    match chained {
        Some(chained) => LunaModelError::WithCause(Box::new(inner), Arc::new(chained)),
        None => inner,
    }
}
