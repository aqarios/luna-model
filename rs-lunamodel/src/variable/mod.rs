//! Owning wrapper over a LunaModel var handle.
//!
//! Mirrors the Rust `Variable`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Variable` still return `Result` here.

use lunamodel_cext_vtable::LmVar;
use rs_lunamodel_derive::RawHandle;


/// Owning handle to a LunaModel var (frees on drop).
#[derive(RawHandle)]
pub struct Variable {
    handle: *mut LmVar,
}

impl Drop for Variable {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, var_free)
        }
    }
}
