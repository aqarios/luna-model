//! Owning wrapper over a LunaModel quadratic handle.
//!
//! Mirrors the Rust `Quadratic`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Quadratic` still return `Result` here.

use lunamodel_cext_vtable::LmQuadratic;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel quadratic (frees on drop).
#[derive(RawHandle)]
pub struct Quadratic {
    handle: *mut LmQuadratic,
}

impl Drop for Quadratic {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, quadratic_free)
        }
    }
}

impl PartialEq for Quadratic {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(quadratic_eq(@self, @other), false)
    }
}
