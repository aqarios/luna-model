use lunamodel_cext_vtable::{LmBuffer, LmVtype};
use lunamodel_types::Vtype;

use super::VarRef;
use crate::traits::LmInto;

use crate::{Result, traits::TryToString};

impl VarRef {
    pub fn id(&self) -> Result<u32> {
        out!(vref_id(@self))
    }

    pub fn check_living(&self) -> Result<bool> {
        out!(vref_check_living(@self))
    }

    /// Returns the variable name.
    ///
    /// Mirrors the native Rust `VarRef::name()` API, but returns `Result` because
    /// the C ABI can fail while resolving the name.
    pub fn name(&self) -> Result<String> {
        out!(vref_name(@self), |out: LmBuffer| out.try_to_string())
    }

    /// Returns the variable type.
    ///
    /// Mirrors the native Rust `VarRef::vtype()` API, converting the C ABI enum into
    /// the shared `lunamodel_types::Vtype`.
    pub fn vtype(&self) -> Result<Vtype> {
        out!(vref_vtype(@self), |out: LmVtype| Ok(out.lm_into()))
    }

    pub fn hash(&self) -> Result<u64> {
        out!(vref_hash(@self))
    }

    // pub vref_bounds: unsafe extern "C" fn(*const LmVref, *mut *mut LmBounds) -> LmStatus,
    // pub vref_inverted: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
}
