//! Owning wrapper over a LunaModel specs handle.
//!
//! Mirrors the Rust `Specs`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Specs` still return `Result` here.

use lunamodel_cext_vtable::LmSpecs;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel specs (frees on drop).
#[derive(RawHandle)]
pub struct Specs {
    handle: *mut LmSpecs,
}

impl Drop for Specs {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, specs_free)
        }
    }
}
