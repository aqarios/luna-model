//! Owning wrapper over a LunaModel specs handle.
//!
//! Mirrors the Rust `Specs`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Specs` still return `Result` here.

use lunamodel_cext_vtable::{LmCtype, LmSpecs, LmVtype};
use lunamodel_types::{Ctype, EnumSet, Sense, Vtype};
use rs_lunamodel_derive::RawHandle;

use crate::Result;
use crate::traits::LmInto;

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

//  pub specs_sense: unsafe extern "C" fn(*const LmSpecs, *mut LmSense) -> LmStatus,
//  pub specs_vtypes: unsafe extern "C" fn(*const LmSpecs, *mut LmVtypeBuffer) -> LmStatus,
//  pub specs_ctypes: unsafe extern "C" fn(*const LmSpecs, *mut LmCtypeBuffer) -> LmStatus,
//  pub specs_max_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
//  pub specs_max_constraint_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
//  pub specs_max_num_variables: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
//  pub specs_satisfies: unsafe extern "C" fn(*const LmSpecs, *const LmSpecs, *mut bool) -> LmStatus,
//  pub specs_clone: unsafe extern "C" fn(*const LmSpecs, *mut *mut LmSpecs) -> LmStatus,
