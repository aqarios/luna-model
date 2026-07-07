//! Owning wrapper over a LunaModel cc handle.
//!
//! Mirrors the Rust `ConstraintCollection`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `ConstraintCollection` still return `Result` here.

use lunamodel_cext_vtable::LmCC;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel cc (frees on drop).
#[derive(RawHandle)]
pub struct ConstraintCollection {
    handle: *mut LmCC,
}

impl Drop for ConstraintCollection {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, cc_free)
        }
    }
}
