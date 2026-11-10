//! Error handling for the C ABI: status codes + a thread-local last structured error.
//!
//! Every non-`Ok` [`LmStatus`] rides alongside a structured [`RichError`] (kind tag + message +
//! cause chain), read back via [`lm_last_error`] and the `lm_error_*` accessors. This is what
//! preserves a [`LunaModelError`]'s variant identity and cause chain across the C ABI instead of
//! flattening it into an opaque string.
#![allow(dead_code)]

use std::os::raw::c_char;
use std::panic::{self, AssertUnwindSafe};

use lunamodel_capi::error::{self, RichError};
use lunamodel_error::LunaModelError;

use crate::LmError;
use crate::LmStatus;
use crate::utils::{drop_from_c, handle_to_c, out_from_c, ref_from_c, str_from_c};
use crate::{LmBuffer, buffer::buffer_from_vec};

/// Internal error carrying the status code, flat message, and (when available) the structured
/// error to surface to callers.
pub(crate) struct CapiError {
    pub(crate) status: LmStatus,
    pub(crate) message: String,
    pub(crate) rich: Option<RichError>,
}

impl CapiError {
    pub(crate) fn new(status: LmStatus, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            rich: None,
        }
    }

    pub(crate) fn null_pointer(what: &str) -> Self {
        Self::new(LmStatus::NullPointer, format!("`{what}` must not be null"))
    }

    /// Maps a Rust error to `LmStatus::Errored`.
    pub(crate) fn errored(e: impl ToString) -> Self {
        Self::new(LmStatus::Errored, e.to_string())
    }

    /// Maps a Rust error to `LmStatus::InvalidArgument`.
    pub(crate) fn invalid_argument(e: impl ToString) -> Self {
        Self::new(LmStatus::InvalidArgument, e.to_string())
    }

    /// Maps a Rust error to `LmStatus::Serialization`.
    pub(crate) fn serialization(e: impl ToString) -> Self {
        Self::new(LmStatus::Serialization, e.to_string())
    }

    /// Maps a Rust error to `LmStatus::Deserialization`.
    pub(crate) fn deserialization(e: impl ToString) -> Self {
        Self::new(LmStatus::Deserialization, e.to_string())
    }
}

/// Builds the [`RichError`] chain for a [`LunaModelError`], preserving its variant (via
/// [`LunaModelError::kind_tag`]) and its full `WithCause` chain instead of flattening either.
fn rich_from_lme(e: &LunaModelError) -> RichError {
    match e {
        LunaModelError::WithCause(inner, cause) => {
            let inner_rich = rich_from_lme(inner);
            let cause_rich = match cause.downcast_ref::<LunaModelError>() {
                Some(lme) => rich_from_lme(lme),
                None => RichError::new("external", cause.to_string()),
            };
            inner_rich.caused_by(cause_rich)
        }
        other => RichError::new(other.kind_tag(), other.to_string()),
    }
}

impl From<LunaModelError> for CapiError {
    /// Preserves the error's variant and cause chain in `rich`, in addition to the flattened
    /// `message` every `CapiError` already carries (so existing string-only consumers keep
    /// working unchanged).
    fn from(e: LunaModelError) -> Self {
        Self {
            status: LmStatus::Errored,
            message: e.to_string(),
            rich: Some(rich_from_lme(&e)),
        }
    }
}

/// Runs `f`, translating its outcome into an [`LmStatus`] and (on failure) recording the
/// thread-local last structured error. Shared by [`guard`] and [`guard_readonly`]; see those for
/// the difference in whether the last-error state is cleared *before* `f` runs.
fn run_guarded(f: impl FnOnce() -> Result<(), CapiError>) -> LmStatus {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(())) => LmStatus::Ok,
        Ok(Err(err)) => {
            error::set_last(
                err.rich
                    .unwrap_or_else(|| RichError::new("errored", err.message)),
            );
            err.status
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "panic in LunaModel C API".to_string());
            error::set_last(RichError::new("panic", msg));
            LmStatus::Panic
        }
    }
}

/// Guards a fallible, top-level C API entry point: clears the thread-local last-error state
/// before running `f`, so a stale error from an unrelated earlier call is never misattributed to
/// this one, then records a fresh one if `f` fails.
pub(crate) fn guard(f: impl FnOnce() -> Result<(), CapiError>) -> LmStatus {
    error::clear_last();
    run_guarded(f)
}

/// Guards a read-only accessor on an already-owned handle (e.g. `lm_error_kind`). Unlike
/// [`guard`], this does *not* clear the last-error state up front: the accessor isn't reporting a
/// fresh top-level failure, it's just reading fields off a value the caller already holds, so it
/// must not have the side effect of wiping `lm_last_error` out from under an unrelated in-flight
/// inspection. On failure (e.g. a null handle), the failure is still recorded like any other C
/// API error.
pub(crate) fn guard_readonly(f: impl FnOnce() -> Result<(), CapiError>) -> LmStatus {
    run_guarded(f)
}

/// Returns the current thread's last structured error as a new owned handle, or null if none is
/// set. Must be released with [`lm_error_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_last_error() -> *mut LmError {
    match error::last() {
        Some(rich) => handle_to_c(rich),
        None => std::ptr::null_mut(),
    }
}

/// Sets the current thread's last structured error. Intended for plugin/extension SDKs to report
/// a pass failure with full fidelity instead of just a non-`Ok` status code.
///
/// `kind` and `message` are copied immediately. `cause` (if non-null) is consumed — the caller
/// must not use or free it afterwards.
///
/// # Safety
/// `kind` and `message` must be null or valid NUL-terminated UTF-8 C strings. `cause`, if
/// non-null, must be a valid error handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_set_last_error(
    kind: *const c_char,
    message: *const c_char,
    cause: *mut LmError,
) -> LmStatus {
    guard(|| {
        let kind = unsafe { str_from_c(kind, "kind") }?;
        let message = unsafe { str_from_c(message, "message") }?;
        let mut rich = RichError::new(kind, message);
        if !cause.is_null() {
            let cause = unsafe { *Box::from_raw(cause as *mut RichError) };
            rich = rich.caused_by(cause);
        }
        error::set_last(rich);
        Ok(())
    })
}

/// Frees an error handle. Passing null is a no-op.
///
/// # Safety
/// `err` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_error_free(err: *mut LmError) {
    unsafe { drop_from_c(err as *mut RichError) }
}

/// Writes the error's kind tag into `out`.
///
/// # Safety
/// `err` must be a valid error handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_error_kind(err: *const LmError, out: *mut LmBuffer) -> LmStatus {
    guard_readonly(|| {
        let err = unsafe { ref_from_c(err as *const RichError, "err") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = buffer_from_vec(err.kind.clone().into_bytes());
        Ok(())
    })
}

/// Writes the error's message into `out`.
///
/// # Safety
/// `err` must be a valid error handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_error_message(err: *const LmError, out: *mut LmBuffer) -> LmStatus {
    guard_readonly(|| {
        let err = unsafe { ref_from_c(err as *const RichError, "err") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = buffer_from_vec(err.message.clone().into_bytes());
        Ok(())
    })
}

/// Returns the error's cause as a new owned handle in `out`, or `LmStatus::Absent` if this error
/// has no cause.
///
/// # Safety
/// `err` must be a valid error handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_error_cause(err: *const LmError, out: *mut *mut LmError) -> LmStatus {
    guard_readonly(|| {
        let err = unsafe { ref_from_c(err as *const RichError, "err") }?;
        let out = unsafe { out_from_c(out) }?;
        match &err.cause {
            Some(cause) => {
                *out = handle_to_c((**cause).clone());
                Ok(())
            }
            None => Err(CapiError::new(LmStatus::Absent, "error has no cause")),
        }
    })
}
