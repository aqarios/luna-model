//! Owning wrapper over a LunaModel sol handle.
//!
//! Mirrors the Rust `Solution`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Solution` still return `Result` here.

use lunamodel_cext_vtable::LmSol;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel sol (frees on drop).
#[derive(RawHandle)]
pub struct Solution {
    handle: *mut LmSol,
}

impl Drop for Solution {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, sol_free)
        }
    }
}
