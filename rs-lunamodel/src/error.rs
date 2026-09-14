//! Error handling mirroring the C ABI `LmStatus` codes.

use std::fmt;

use lunamodel_capi::{LmBuffer, LmError, LmStatus};

use crate::api::api;
use crate::traits::TryToString;
pub use crate::error_kind::ErrorKind;

/// Result type returned by fallible C-ABI calls.
pub type Result<T> = std::result::Result<T, Error>;

/// Native-compatible name for [`Result`].
pub type LunaModelResult<T> = Result<T>;

/// Native-compatible name for [`Error`].
pub type LunaModelError = Error;

/// A failure returned across the C ABI, mirroring the non-`Ok` `LmStatus` codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// A required pointer argument was null.
    NullPointer,
    /// An argument was invalid.
    InvalidArgument,
    /// Encoding a value to bytes failed.
    Serialization,
    /// Decoding a value from bytes failed.
    Deserialization,
    /// A Rust panic was caught at the FFI boundary.
    Panic,
    /// An error was encountered.
    Errored,
    /// A requested optional value was not present.
    Absent,
}

/// An error surfaced from the LunaModel C ABI: a status code plus the
/// thread-local last-error message captured at the call site.
#[derive(Debug, Clone)]
pub struct Error {
    /// The failing status code.
    pub status: Status,
    /// The associated message, if any.
    pub message: String,
    /// The raw, open string tag identifying this error's specific origin — either a native
    /// LunaModel error's [`kind_tag`](lunamodel_error::LunaModelError::kind_tag) (e.g.
    /// `"variable_not_existing"`), a plugin-defined kind (e.g.
    /// `"myplugin::infeasible_subproblem"`), or a coarse status-derived fallback (e.g.
    /// `"errored"`) when nothing more specific was reported. `None` only when no structured error
    /// was available at all. Prefer [`Error::kind`] for a typed, matchable classification.
    pub kind_tag: Option<String>,
}

impl Error {
    /// Creates an error for adapter-side validation or callback failures.
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            kind_tag: None,
        }
    }

    /// Creates a generic error for a failed external computation.
    pub fn computation(message: impl Into<String>) -> Self {
        Self::new(Status::Errored, message)
    }

    /// Creates an error for an unexpected adapter-side state.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(Status::Panic, message)
    }

    /// Attaches a specific kind tag, reported in place of the generic status-derived one.
    #[must_use]
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind_tag = Some(kind.into());
        self
    }

    /// This error's domain/business-logic kind, typed and matchable — see [`ErrorKind`] for why
    /// it's a closed enum with an [`ErrorKind::Other`] fallback rather than the raw tag directly.
    pub fn kind(&self) -> ErrorKind {
        ErrorKind::from_tag(self.kind_tag.as_deref().unwrap_or(""))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "{:?}", self.status)
        } else {
            write!(f, "{:?}: {}", self.status, self.message)
        }
    }
}

impl std::error::Error for Error {}

impl Status {
    /// Maps a status code to its typed variant, or `None` for `Ok`.
    fn from_code(status: LmStatus) -> Option<Self> {
        match status {
            LmStatus::Ok => None,
            LmStatus::NullPointer => Some(Status::NullPointer),
            LmStatus::InvalidArgument => Some(Status::InvalidArgument),
            LmStatus::Serialization => Some(Status::Serialization),
            LmStatus::Deserialization => Some(Status::Deserialization),
            LmStatus::Panic => Some(Status::Panic),
            LmStatus::Errored => Some(Status::Errored),
            LmStatus::Absent => Some(Status::Absent),
        }
    }

    /// Maps back to the non-`Ok` status code a trampoline should return for this status.
    fn to_code(self) -> LmStatus {
        match self {
            Status::NullPointer => LmStatus::NullPointer,
            Status::InvalidArgument => LmStatus::InvalidArgument,
            Status::Serialization => LmStatus::Serialization,
            Status::Deserialization => LmStatus::Deserialization,
            Status::Panic => LmStatus::Panic,
            Status::Errored => LmStatus::Errored,
            Status::Absent => LmStatus::Absent,
        }
    }

    /// A stable kind tag for this status, used to populate `RichError::kind` when a pass author's
    /// error doesn't otherwise carry a more specific one.
    fn as_kind(self) -> &'static str {
        match self {
            Status::NullPointer => "null_pointer",
            Status::InvalidArgument => "invalid_argument",
            Status::Serialization => "serialization",
            Status::Deserialization => "deserialization",
            Status::Panic => "panic",
            Status::Errored => "errored",
            Status::Absent => "absent",
        }
    }
}

/// Reports a pass author's error to the host via `lm_set_last_error` before a trampoline returns
/// its status, so the error's message (and, going forward, richer context) crosses the ABI
/// instead of being silently discarded. Returns the status code the trampoline should return.
pub(crate) fn report(e: &Error) -> LmStatus {
    let kind_str = e.kind_tag.as_deref().unwrap_or_else(|| e.status.as_kind());
    let kind = std::ffi::CString::new(kind_str).unwrap_or_default();
    let message = std::ffi::CString::new(e.message.replace('\0', " ")).unwrap_or_default();
    unsafe { (api().set_last_error)(kind.as_ptr(), message.as_ptr(), std::ptr::null_mut()) };
    e.status.to_code()
}

/// Turns a status code into `Ok(())` or an [`Error`] carrying the thread-local last structured
/// error's message and kind.
pub(crate) fn check(status: LmStatus) -> Result<()> {
    match Status::from_code(status) {
        None => Ok(()),
        Some(status) => Err(last_error(status)),
    }
}

/// Like [`check`], but treats `Absent` as a successful "not present".
///
/// Returns `Ok(true)` for `Ok`, `Ok(false)` for `Absent`, and `Err` for any real
/// failure. Used by `out_opt!` to map an absent value to `Ok(None)`.
pub(crate) fn present(status: LmStatus) -> Result<bool> {
    match status {
        LmStatus::Absent => Ok(false),
        other => check(other).map(|()| true),
    }
}

/// Builds an [`Error`] for a failing `status`, reading the message and kind tag from the
/// thread-local [`RichError`](lunamodel_capi::error::RichError) the host recorded for this call —
/// the same structured accessors (`lm_last_error`/`lm_error_*`) a plugin's own reported errors are
/// read back through, so an ordinary host-side failure carries just as much fidelity as one a
/// plugin author reported via `lm_set_last_error`.
fn last_error(status: Status) -> Error {
    let err: *mut LmError = (api().last_error)();
    if err.is_null() {
        return Error {
            status,
            message: String::new(),
            kind_tag: None,
        };
    }
    let message = read_buffer(|out| unsafe { (api().error_message)(err, out) }).unwrap_or_default();
    let kind_tag = read_buffer(|out| unsafe { (api().error_kind)(err, out) });
    unsafe { (api().error_free)(err) };
    Error {
        status,
        message,
        kind_tag,
    }
}

/// Runs a `lm_error_*(err, &mut LmBuffer)`-shaped accessor and converts its output buffer to an
/// owned `String`, or `None` if the accessor itself reports failure (e.g. `LmStatus::Absent`).
fn read_buffer(f: impl FnOnce(&mut LmBuffer) -> LmStatus) -> Option<String> {
    let mut buf = LmBuffer::default();
    if f(&mut buf) != LmStatus::Ok {
        return None;
    }
    buf.try_to_string().ok()
}
