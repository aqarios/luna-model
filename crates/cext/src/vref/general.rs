use luna_model::{
    core::prelude::{VarRef, Variable},
    error::LunaModelResult,
};

use crate::{
    LmStatus, LmVar, LmVref,
    error::{CapiError, guard},
};

/// Compares the variable index and the semantic contents of the environment.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `other` must be a valid vref handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_eq(
    vref: *const LmVref,
    other: *const LmVref,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;
        let other = unsafe { (other as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = vref.eq(other);
        Ok(())
    })
}

/// Materializes the vref as an owned LmVar.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmVar` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_to_var(vref: *const LmVref, out: *mut *mut LmVar) -> LmStatus {
    guard(|| {
        let vref = unsafe { (vref as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("vref"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let maybe_variable: LunaModelResult<Variable> = vref.try_into();
        let variable =
            maybe_variable.map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(variable)) as *mut LmVar;
        Ok(())
    })
}
