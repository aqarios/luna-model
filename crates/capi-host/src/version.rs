//! Library and ABI version queries.

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::CAPI_ABI_VERSION;

/// Returns the LunaModel version string (the crate's `CARGO_PKG_VERSION`).
///
/// The returned pointer is a static, NUL-terminated UTF-8 string owned by the
/// library and valid for the lifetime of the program. Do not free it.
#[unsafe(no_mangle)]
pub extern "C" fn lm_version() -> *const c_char {
    // Compile-time NUL-terminated version string; no allocation, no lifetime issues.
    const VERSION: &CStr =
        match CStr::from_bytes_with_nul(concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes()) {
            Ok(s) => s,
            Err(_) => c"unknown",
        };
    VERSION.as_ptr()
}

/// Returns the ABI version of the C API table (see [`crate::CAPI_ABI_VERSION`]).
///
/// Consumers that obtain the table through the Python capsule should compare this
/// against the `CAPI_ABI_VERSION` they were compiled against before making calls.
#[unsafe(no_mangle)]
pub extern "C" fn lm_abi_version() -> u32 {
    CAPI_ABI_VERSION
}
