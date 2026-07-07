//! Owning wrapper over a LunaModel higher_order handle.
//!
//! Mirrors the Rust `HigherOrder`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `HigherOrder` still return `Result` here.

use lunamodel_cext_vtable::LmHigherOrder;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel higher_order (frees on drop).
#[derive(RawHandle)]
pub struct HigherOrder {
    handle: *mut LmHigherOrder,
}

impl Drop for HigherOrder {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, higher_order_free)
        }
    }
}

impl PartialEq for HigherOrder {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(higher_order_eq(@self, @other), false)
    }
}
