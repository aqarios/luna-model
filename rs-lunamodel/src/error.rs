//! Error handling mirroring the C ABI `LmStatus` codes.

use std::ffi::CStr;
use std::fmt;

use lunamodel_cext_vtable::LmStatus;

use crate::api::api;

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
}

impl Error {
    /// Creates an error for adapter-side validation or callback failures.
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
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
}

/// Turns a status code into `Ok(())` or an [`Error`] carrying the thread-local
/// last-error message.
pub(crate) fn check(status: LmStatus) -> Result<()> {
    match Status::from_code(status) {
        None => Ok(()),
        Some(status) => Err(Error {
            status,
            message: last_error_message(),
        }),
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

/// Reads the thread-local last-error message set by the most recent C call.
fn last_error_message() -> String {
    let ptr = (api().last_error_message)();
    if ptr.is_null() {
        String::new()
    } else {
        // SAFETY: the table returns a pointer to a NUL-terminated string owned by
        // the host and valid until the next call on this thread.
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}
