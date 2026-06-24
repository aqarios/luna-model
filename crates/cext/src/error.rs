//! Error handling for the C ABI: status codes + a thread-local last-error message.
//!
//! Some helpers here are first used by later modules; the crate-level allow keeps
//! incremental builds warning-free and can be dropped once everything is wired up.
#![allow(dead_code)]

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::{self, AssertUnwindSafe};

/// Status codes returned by every fallible `lm_*` entry point.
///
/// `Ok` is guaranteed to be `0`. On any non-`Ok` status, a human-readable
/// description is available from [`lm_last_error_message`] until the next C API
/// call on the same thread.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmStatus {
    /// Operation succeeded.
    Ok = 0,
    /// A required pointer argument was null.
    NullPointer = 1,
    /// An argument was invalid (e.g. non-UTF-8 bytes where text was required).
    InvalidArgument = 2,
    /// Encoding a value to bytes failed.
    Serialization = 3,
    /// Decoding a value from bytes failed.
    Deserialization = 4,
    /// A Rust panic was caught at the FFI boundary.
    Panic = 5,
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

/// Internal error carrying the status code and message to surface to callers.
pub(crate) struct CapiError {
    pub(crate) status: LmStatus,
    pub(crate) message: String,
}

impl CapiError {
    pub(crate) fn new(status: LmStatus, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub(crate) fn null_pointer(what: &str) -> Self {
        Self::new(LmStatus::NullPointer, format!("`{what}` must not be null"))
    }
}

fn set_last_error(message: &str) {
    // Replace interior NULs so the message survives the CString conversion.
    let cstring = CString::new(message.replace('\0', " ")).unwrap_or_default();
    LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(cstring));
}

/// Runs `f`, catching panics and recording any error message for the caller.
///
/// Returns [`LmStatus::Ok`] on success. On a returned error or a caught panic the
/// message is stored thread-locally (see [`lm_last_error_message`]) and the
/// matching status code is returned, so a panic never unwinds across the boundary.
pub(crate) fn guard(f: impl FnOnce() -> Result<(), CapiError>) -> LmStatus {
    LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(())) => LmStatus::Ok,
        Ok(Err(err)) => {
            set_last_error(&err.message);
            err.status
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "panic in LunaModel C API".to_string());
            set_last_error(&msg);
            LmStatus::Panic
        }
    }
}

/// Returns the last error message for the current thread, or null if there is none.
///
/// The pointer is owned by LunaModel and stays valid only until the next C API
/// call on this thread. Copy it if you need to keep it; do not free it.
///
/// # Safety
/// The caller must not retain the returned pointer across further C API calls.
#[unsafe(no_mangle)]
pub extern "C" fn lm_last_error_message() -> *const c_char {
    LAST_ERROR.with(|slot| match &*slot.borrow() {
        Some(cstring) => cstring.as_ptr(),
        None => std::ptr::null(),
    })
}
