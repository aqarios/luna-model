//! Owning wrapper over a LunaModel linear handle.
//!
//! Mirrors the Rust `Linear`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Linear` still return `Result` here.

use lunamodel_cext_vtable::LmLinear;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel linear (frees on drop).
#[derive(RawHandle)]
pub struct Linear {
    handle: *mut LmLinear,
}

impl Drop for Linear {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, linear_free)
        }
    }
}

impl PartialEq for Linear {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(linear_eq(@self, @other), false)
    }
}
