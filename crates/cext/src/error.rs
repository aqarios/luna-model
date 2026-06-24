//! Error handling for the C ABI: status codes + a thread-local last-error message.
#![allow(dead_code)]

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::{self, AssertUnwindSafe};

use crate::LmStatus;

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
    let cstring = CString::new(message.replace('\0', " ")).unwrap_or_default();
    LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(cstring));
}

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
