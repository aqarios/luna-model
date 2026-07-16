//! Owning wrapper over a LunaModel constr handle.
//!
//! Mirrors the Rust `Constraint`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Constraint` still return `Result` here.

mod access;
mod modification;

use lunamodel_cext_vtable::LmConstraint;
use lunamodel_types::{Bias, Comparator};
use rs_lunamodel_derive::RawHandle;

use crate::{ContentEquality, Env, Expression, Result, TryClone, TryDeepClone};

/// Owning handle to a LunaModel constr (frees on drop).
#[derive(RawHandle)]
pub struct Constraint {
    handle: *mut LmConstraint,
}

impl Drop for Constraint {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, constr_free)
        }
    }
}

impl TryClone for Constraint {
    fn try_clone(&self) -> crate::Result<Self> {
        out!(constr_clone(@self))
    }
}

impl<'e> TryDeepClone<'e> for Constraint {
    type Target = &'e Env;

    fn try_deep_clone(&self, target: Self::Target) -> crate::Result<Self> {
        out!(constr_deep_clone(@self, @target))
    }
}

impl Constraint {
    pub fn new(
        lhs: Expression,
        rhs: Bias,
        comparator: Comparator,
        name: Option<String>,
    ) -> Result<Self> {
        out!(constr_new(@lhs, rhs, ~comparator, ?!name))
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        out_or!(constr_eq(@self, @other), false)
    }
}

impl ContentEquality for Constraint {
    fn equal_contents(&self, other: &Self) -> Result<bool> {
        out!(constr_equal_contents(@self, @other))
    }
}

impl std::fmt::Debug for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraint")
            .field("name", &self.name().ok())
            .field("auto_name", &self.auto_name().ok())
            .field("rhs", &self.rhs().ok())
            .field("comparator", &self.comparator().ok())
            .finish()
    }
}

// SAFETY: every operation on an LmConstraint is synchronized by the host.
unsafe impl Send for Constraint {}
unsafe impl Sync for Constraint {}
