use lunamodel_core::prelude::{VarRef, Variable};
use lunamodel_error::LunaModelResult;

use crate::{
    LmStatus, LmVar, LmVref,
    error::{CapiError, guard},
    utils::{handle_to_c, out_from_c, ref_from_c},
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
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let other = unsafe { ref_from_c(other as *const VarRef, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = vref.eq(other);
        Ok(())
    })
}

/// Materializes the vref as an owned LmVar.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmVar *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_to_var(vref: *const LmVref, out: *mut *mut LmVar) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        let maybe_variable: LunaModelResult<Variable> = vref.try_into();
        let variable = maybe_variable.map_err(CapiError::from)?;
        *out = handle_to_c(variable);
        Ok(())
    })
}

/// Clones a variable reference.
///
/// # Safety
/// `vref` must be a valid variable-reference handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_clone(vref: *const LmVref, out: *mut *mut LmVref) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(vref.clone());
        Ok(())
    })
}
