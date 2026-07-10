//! Owning wrapper over a LunaModel var handle.
//!
//! Mirrors the Rust `Variable`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Variable` still return `Result` here.

use lunamodel_cext_vtable::LmVar;
use lunamodel_types::Vtype;
use rs_lunamodel_derive::RawHandle;

use crate::{Bounds, Result};

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

impl Variable {
    pub fn name(&self) -> Result<String> {
        out!(var_name(@self))
    }

    pub fn vtype(&self) -> Result<Vtype> {
        out!(var_vtype(@self))
    }

    pub fn bounds(&self) -> Result<Bounds> {
        out!(var_bounds(@self))
    }

    pub fn inverted(&self) -> Result<u32> {
        out!(var_inverted(@self))
    }
}
