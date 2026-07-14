//! Owning wrapper over a LunaModel specs handle.
//!
//! Mirrors the Rust `Specs`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Specs` still return `Result` here.

use lunamodel_cext_vtable::{LmCtype, LmSpecs, LmVtype};
use lunamodel_types::{Ctype, EnumSet, Sense, Vtype};
use rs_lunamodel_derive::RawHandle;

use crate::traits::LmInto;
use crate::{Result, TryClone};

/// Owning handle to a LunaModel specs (frees on drop).
#[derive(RawHandle)]
pub struct Specs {
    handle: *mut LmSpecs,
}

impl Drop for Specs {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, specs_free)
        }
    }
}

impl TryClone for Specs {
    fn try_clone(&self) -> Result<Self> {
        out!(specs_clone(@self))
    }
}

impl Specs {
    pub fn new(
        sense: Option<Sense>,
        vtypes: Option<EnumSet<Vtype>>,
        constraints: Option<EnumSet<Ctype>>,
        max_degree: Option<usize>,
        max_constraint_degree: Option<usize>,
        max_num_variables: Option<usize>,
    ) -> Result<Self> {
        // A null array means "unspecified" (`None`); `Some(EnumSet::empty())`
        // still yields a non-null pointer with len 0, an explicit empty set.
        let vtypes: Option<Vec<LmVtype>> = vtypes.map(|s| s.iter().map(|v| v.lm_into()).collect());
        let constraints: Option<Vec<LmCtype>> =
            constraints.map(|s| s.iter().map(|c| c.lm_into()).collect());

        out!(specs_new(
            ?~sense,
            ?*vtypes,
            ?*constraints,
            ?max_degree,
            ?max_constraint_degree,
            ?max_num_variables,
        ))
    }
}

impl Specs {
    pub fn sense(&self) -> Result<Option<Sense>> {
        out_opt!(specs_sense(@self))
    }

    pub fn vtypes(&self) -> Result<Option<EnumSet<Vtype>>> {
        out_opt!(specs_vtypes(@self))
    }

    pub fn ctypes(&self) -> Result<Option<EnumSet<Ctype>>> {
        out_opt!(specs_ctypes(@self))
    }

    pub fn max_degree(&self) -> Result<Option<usize>> {
        out_opt!(specs_max_degree(@self))
    }

    pub fn max_constraint_degree(&self) -> Result<Option<usize>> {
        out_opt!(specs_max_constraint_degree(@self))
    }

    pub fn max_num_variables(&self) -> Result<Option<usize>> {
        out_opt!(specs_max_num_variables(@self))
    }

    pub fn satisfies(&self, other: &Specs) -> Result<bool> {
        out!(specs_satisfies(@self, @other))
    }

}

impl PartialEq for Specs {
    fn eq(&self, other: &Self) -> bool {
        self.sense().ok() == other.sense().ok()
            && self.vtypes().ok() == other.vtypes().ok()
            && self.ctypes().ok() == other.ctypes().ok()
            && self.max_degree().ok() == other.max_degree().ok()
            && self.max_constraint_degree().ok() == other.max_constraint_degree().ok()
            && self.max_num_variables().ok() == other.max_num_variables().ok()
    }
}

impl std::fmt::Debug for Specs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Specs")
            .field("sense", &self.sense().ok().flatten())
            .field("vtypes", &self.vtypes().ok().flatten())
            .field("constraints", &self.ctypes().ok().flatten())
            .field("max_degree", &self.max_degree().ok().flatten())
            .field(
                "max_constraint_degree",
                &self.max_constraint_degree().ok().flatten(),
            )
            .field(
                "max_num_variables",
                &self.max_num_variables().ok().flatten(),
            )
            .finish()
    }
}

impl std::fmt::Display for Specs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModelSpecs(sense={:?}, vtype={:?}, constraints={:?}, max_degree={:?}, max_constraint_degree={:?}, max_num_variables={:?})",
            self.sense().ok().flatten(),
            self.vtypes().ok().flatten(),
            self.ctypes().ok().flatten(),
            self.max_degree().ok().flatten(),
            self.max_constraint_degree().ok().flatten(),
            self.max_num_variables().ok().flatten()
        )
    }
}
