//! FFI marshaling helpers for driving plugin callbacks across the C boundary.
//!
//! These keep [`pass`](crate::pass) focused on the trait implementations: string/byte collection
//! via sink callbacks (so ownership never crosses the ABI) and moving values back out of the
//! owned handles lent to the plugin.

use std::ffi::{CStr, c_char, c_void};
use std::sync::Arc;

use lunamodel_capi::plugin::LmStrSink;
use lunamodel_capi_handle::{Shared, SharedHandleKind, handle_into_shared};
use lunamodel_transpiler::{TranspileErrorKind, TranspileKindResult};

/// Copies a plugin-owned, borrowed C string into an owned `String`.
pub(crate) unsafe fn borrow_cstr(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

/// Drives a plugin string-list callback, collecting the emitted strings.
pub(crate) unsafe fn collect_strings(
    user_data: *mut c_void,
    f: unsafe extern "C" fn(*mut c_void, LmStrSink, *mut c_void),
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    unsafe {
        f(
            user_data,
            str_sink,
            (&mut out as *mut Vec<String>).cast::<c_void>(),
        )
    };
    out
}

/// `LmStrSink`: pushes each emitted string into a `Vec<String>`.
unsafe extern "C" fn str_sink(ctx: *mut c_void, s: *const c_char) {
    if ctx.is_null() || s.is_null() {
        return;
    }
    let out = unsafe { &mut *(ctx as *mut Vec<String>) };
    out.push(unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned());
}

/// `LmByteSink`: appends emitted bytes into a `Vec<u8>`.
pub(crate) unsafe extern "C" fn byte_sink(ctx: *mut c_void, data: *const u8, len: usize) {
    if ctx.is_null() || data.is_null() || len == 0 {
        return;
    }
    let out = unsafe { &mut *(ctx as *mut Vec<u8>) };
    out.extend_from_slice(unsafe { std::slice::from_raw_parts(data, len) });
}

/// Consumes an owned shared handle and moves its value back out, erroring if the plugin retained
/// another owning reference to it.
pub(crate) fn reclaim<H, V>(handle: *mut H, what: &str, phase: &str) -> TranspileKindResult<V>
where
    H: SharedHandleKind<Value = V, Storage = Shared<H>>,
{
    let shared = unsafe { handle_into_shared::<H>(handle) }
        .ok_or_else(|| plugin_error(&format!("null {what} handle after {phase}")))?;
    Arc::into_inner(shared)
        .map(|lock| lock.into_inner())
        .ok_or_else(|| plugin_error(&format!("{what} reference leaked out of plugin {phase}")))
}

/// Builds a transpiler error from a plugin-boundary failure.
///
/// Plugin SDKs report their own failure via `lm_set_last_error` before returning a non-`Ok`
/// status; callers here clear the thread-local right before invoking the plugin callback (so a
/// stale error from an unrelated earlier call is never misattributed), then check it once the
/// callback returns non-`Ok`. When the plugin did report one, that structured error (kind,
/// message, and its cause chain — [`lunamodel_capi::error::RichError`] implements
/// [`std::error::Error`]) becomes the *cause* here, so it folds into the same
/// `LunaModelError::WithCause` chain a native pass failure would produce, rather than being
/// replaced by a generic message. `fallback` covers plugins (e.g. minimal C authors) that don't
/// call `lm_set_last_error` and only return the status code.
pub(crate) fn plugin_error(fallback: &str) -> TranspileErrorKind {
    match lunamodel_capi::error::last() {
        Some(rich) => TranspileErrorKind::external_with_kind(rich.kind.clone(), rich),
        None => TranspileErrorKind::external(std::io::Error::other(fallback.to_string())),
    }
}
