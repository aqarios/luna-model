//! Owning wrapper over a LunaModel vref handle.
//!
//! Mirrors the Rust `VarRef`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `VarRef` still return `Result` here.

mod access;
mod arithmetic;
mod creation;
mod general;

use lunamodel_cext_vtable::LmVref;
use rs_lunamodel_derive::RawHandle;

use crate::{Result, traits::LmTryFrom};

/// Owning handle to a LunaModel vref (frees on drop).
#[derive(RawHandle)]
pub struct VarRef {
    handle: *mut LmVref,
}

impl Drop for VarRef {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, vref_free)
        }
    }
}

impl PartialEq for VarRef {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(vref_eq(@self, @other), false)
    }
}

impl LmTryFrom<*mut LmVref> for Option<VarRef> {
    fn lm_try_from(value: *mut LmVref) -> Result<Self> {
        Ok((!value.is_null()).then(|| VarRef::from_raw(value)))
    }
}
