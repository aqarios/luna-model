//! Owning wrapper over a LunaModel bounds handle.
//!
//! Mirrors the Rust `Bounds`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Bounds` still return `Result` here.

use luna_model_ffi_derive::RawHandle;
use lunamodel_capi::LmBounds;
use lunamodel_types::Vtype;

use crate::{Bound, Result, TryClone};

/// Owning handle to a LunaModel bounds (frees on drop).
#[derive(RawHandle)]
pub struct Bounds {
    handle: *mut LmBounds,
}

impl Drop for Bounds {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, bounds_free)
        }
    }
}

impl TryClone for Bounds {
    fn try_clone(&self) -> Result<Self> {
        out!(bounds_clone(@self))
    }
}

impl Bounds {
    pub fn new(lower: Bound, upper: Bound) -> Result<Self> {
        out!(bounds_new(~lower, ~upper))
    }

    pub fn binary() -> Result<Self> {
        out!(bounds_binary())
    }

    pub fn spin() -> Result<Self> {
        out!(bounds_spin())
    }

    pub fn integer() -> Result<Self> {
        out!(bounds_integer())
    }

    pub fn real() -> Result<Self> {
        out!(bounds_real())
    }

    pub fn default_for(vtype: Vtype) -> Result<Self> {
        out!(bounds_default_for(~vtype))
    }
}

impl PartialEq for Bounds {
    fn eq(&self, other: &Self) -> bool {
        out_or!(bounds_eq(@self, @other), false)
    }
}

impl std::fmt::Debug for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bounds")
            .field("lower", &self.lower().ok())
            .field("upper", &self.upper().ok())
            .finish()
    }
}

impl Bounds {
    pub fn lower(&self) -> Result<Bound> {
        out!(bounds_lower(@self))
    }

    pub fn upper(&self) -> Result<Bound> {
        out!(bounds_upper(@self))
    }

    pub fn evaluate(&self, val: f64, tol: Option<f64>) -> Result<bool> {
        out!(bounds_eval(@self, val, ?tol))
    }
}
