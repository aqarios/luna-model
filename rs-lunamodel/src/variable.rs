//! Owning wrapper over a LunaModel var handle.
//!
//! Mirrors the Rust `Variable`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Variable` still return `Result` here.

use lunamodel_capi::LmVar;
use lunamodel_types::Vtype;
use rs_lunamodel_derive::RawHandle;

use crate::{Bounds, LazyBounds, Result, TryClone};

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
    pub fn new(name: &str, vtype: Vtype, bounds: Option<&LazyBounds>) -> Result<Self> {
        out!(var_new(!name, ~vtype, ?@bounds))
    }

    pub fn name(&self) -> Result<String> {
        out!(var_name(@self))
    }

    pub fn vtype(&self) -> Result<Vtype> {
        out!(var_vtype(@self))
    }

    pub fn bounds(&self) -> Result<Bounds> {
        out!(var_bounds(@self))
    }

    pub fn inverted(&self) -> Result<Option<u32>> {
        out_opt!(var_inverted(@self))
    }
}

impl TryClone for Variable {
    fn try_clone(&self) -> Result<Self> {
        out!(var_clone(@self))
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        out_or!(var_eq(@self, @other), false)
    }
}

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Variable")
            .field("name", &self.name().ok())
            .field("vtype", &self.vtype().ok())
            .field("bounds", &self.bounds().ok())
            .field("inverted", &self.inverted().ok().flatten())
            .finish()
    }
}
