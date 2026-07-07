//! Owning wrapper over a LunaModel sol handle.
//!
//! Mirrors the Rust `Solution`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Solution` still return `Result` here.

mod access;
mod convenience;
mod creation;
mod modification;

use lunamodel_cext_vtable::LmSol;
use rs_lunamodel_derive::RawHandle;

use crate::{ContentEquality, Result, TryClone, api::api};

/// Owning handle to a LunaModel sol (frees on drop).
#[derive(RawHandle)]
pub struct Solution {
    handle: *mut LmSol,
}

impl Drop for Solution {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, sol_free)
        }
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::from_raw((api().sol_default)())
    }
}

impl TryClone for Solution {
    fn try_clone(&self) -> Result<Self> {
        out!(sol_clone(@self))
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        out_or!(sol_eq(@self, @other), false)
    }
}

impl ContentEquality for Solution {
    fn equal_contents(&self, other: &Self) -> Result<bool> {
        out!(sol_equal_contents(@self, @other))
    }
}
