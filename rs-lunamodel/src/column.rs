//! Owning wrapper over a LunaModel col handle.
//!
//! Mirrors the Rust `Column`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Column` still return `Result` here.

use lunamodel_cext_vtable::LmColumn;
use rs_lunamodel_derive::RawHandle;

use crate::{Result, assignment::Assignment};

/// Owning handle to a LunaModel col (frees on drop).
#[derive(RawHandle)]
pub struct Column {
    handle: *mut LmColumn,
}

impl Drop for Column {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, col_free)
        }
    }
}

impl Column {
    pub fn len(&self) -> Result<usize> {
        out!(col_len(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(col_is_empty(@self))
    }

    pub fn assignment(&self, idx: usize) -> Result<Assignment> {
        out!(col_assignment(@self, idx))
    }
}
