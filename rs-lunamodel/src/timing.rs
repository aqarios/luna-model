//! Owning wrapper over a LunaModel timing handle.
//!
//! Mirrors the Rust `Timing`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Timing` still return `Result` here.

use lunamodel_cext_vtable::LmTiming;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel timing (frees on drop).
#[derive(RawHandle)]
pub struct Timing {
    handle: *mut LmTiming,
}

impl Drop for Timing {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, timing_free)
        }
    }
}
