//! Owning wrapper over a LunaModel vref handle.
//!
//! Mirrors the Rust `VarRef`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Expression` still return `Result` here.

mod access;
mod creation;

use lunamodel_cext_vtable::LmVref;

use crate::api::api;

/// Owning handle to a LunaModel expression (frees on drop).
pub struct VarRef {
    handle: *mut LmVref,
}

impl Drop for VarRef {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // SAFETY: `handle` was produced by this library and is freed once.
            unsafe { (api().vref_free)(self.handle) };
        }
    }
}

impl VarRef {
    /// Wraps a raw handle owned by the caller.
    pub(crate) fn from_raw(handle: *mut LmVref) -> Self {
        VarRef { handle }
    }

    /// Borrows the raw handle for C interop.
    pub fn as_ptr(&self) -> *const LmVref {
        self.handle
    }
}
