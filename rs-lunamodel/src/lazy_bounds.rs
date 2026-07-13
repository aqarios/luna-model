//! Owning wrapper over a LunaModel lazy bounds handle.
//!
//! Mirrors the Rust `LazyBounds`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `LazyBounds` still return `Result` here.

use lunamodel_cext_vtable::LmLazyBounds;
use lunamodel_types::Vtype;
use rs_lunamodel_derive::RawHandle;

use crate::{Bound, Bounds, Result, TryClone, api::api};

/// Owning handle to a LunaModel lazy bounds (frees on drop).
#[derive(RawHandle)]
pub struct LazyBounds {
    handle: *mut LmLazyBounds,
}

impl Drop for LazyBounds {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, lazy_bounds_free)
        }
    }
}

impl Default for LazyBounds {
    fn default() -> Self {
        Self::from_raw((api().lazy_bounds_default)())
    }
}

impl TryClone for LazyBounds {
    fn try_clone(&self) -> Result<Self> {
        out!(lazy_bounds_clone(@self))
    }
}

impl LazyBounds {
    pub fn new(lower: Option<Bound>, upper: Option<Bound>) -> Result<Self> {
        out!(lazy_bounds_new(?~lower, ?~upper))
    }
}

impl LazyBounds {
    pub fn lower(&self) -> Result<Option<Bound>> {
        out_opt!(lazy_bounds_lower(@self))
    }

    pub fn upper(&self) -> Result<Option<Bound>> {
        out_opt!(lazy_bounds_upper(@self))
    }
}

impl LazyBounds {
    pub fn concretize(&self, vtype: Vtype) -> Result<Bounds> {
        out!(lazy_bounds_concretize(@self, ~vtype))
    }
}
