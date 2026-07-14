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

use crate::{ContentEquality, Result, Serialization, TryClone, TryIndex, api::api};

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

impl Serialization<'_> for Solution {
    type Payload = ();

    fn serialize(&self) -> Result<Vec<u8>> {
        out!(sol_serialize(@self))
    }

    fn deserialize(data: &[u8], _: Self::Payload) -> Result<Self> {
        out!(sol_deserialize(@data, data.len()))
    }
}

impl TryIndex<(usize, &str)> for Solution {
    type Output = f64;
    fn try_index(&self, (row, name): (usize, &str)) -> Result<Self::Output> {
        self.value(row, name)?.ok_or_else(|| crate::Error {
            status: crate::Status::Absent,
            message: format!("solution has no value at row {row} for variable '{name}'"),
        })
    }
}

impl TryIndex<(usize, usize)> for Solution {
    type Output = f64;
    fn try_index(&self, (row, column): (usize, usize)) -> Result<Self::Output> {
        Ok(self.assignment_idx(row, column)?.value())
    }
}

impl std::fmt::Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solution")
            .field("variable_names", &self.variable_names().ok())
            .field("counts", &self.counts().ok())
            .field("raw_energies", &self.raw_energies().ok().flatten())
            .field("obj_values", &self.obj_values().ok().flatten())
            .field("feasible", &self.feasible().ok().flatten())
            .field("sense", &self.sense().ok())
            .finish()
    }
}
