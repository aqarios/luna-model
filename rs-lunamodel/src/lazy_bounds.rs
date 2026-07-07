//! Owning wrapper over a LunaModel lazy bounds handle.
//!
//! Mirrors the Rust `LazyBounds`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `LazyBounds` still return `Result` here.

use lunamodel_cext_vtable::LmLazyBounds;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel lazy bounds (frees on drop).
#[derive(RawHandle)]
pub struct LazyBounds {
    handle: *mut LmLazyBounds,
}

impl Drop for LazyBounds {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, lazy_bounds_free)
        }
    }
}
