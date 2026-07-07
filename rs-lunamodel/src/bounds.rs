//! Owning wrapper over a LunaModel bounds handle.
//!
//! Mirrors the Rust `Bounds`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Bounds` still return `Result` here.

use lunamodel_cext_vtable::LmBounds;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel bounds (frees on drop).
#[derive(RawHandle)]
pub struct Bounds {
    handle: *mut LmBounds,
}

impl Drop for Bounds {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, bounds_free)
        }
    }
}
