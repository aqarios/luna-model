mod access;

use std::ffi::c_char;

use luna_model::core::prelude::{LazyBounds, Variable};

use crate::{
    LmLazyBounds, LmStatus, LmVar, LmVtype,
    error::{CapiError, guard},
    utils::{drop_from_c, handle_to_c, out_from_c, ref_from_c, str_from_c, vtype_from_c},
};

/// Frees a variable handle. Passing null is a no-op.
///
/// # Safety
/// `var` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_free(var: *mut LmVar) {
    unsafe { drop_from_c(var as *mut Variable) }
}

/// Creates an owned variable record.
///
/// # Safety
/// `name` must be NUL-terminated, optional `bounds` valid, and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_new(
    name: *const c_char,
    vtype: LmVtype,
    bounds: *const LmLazyBounds,
    out: *mut *mut LmVar,
) -> LmStatus {
    guard(|| {
        let name = unsafe { str_from_c(name, "name") }?;
        let bounds = if bounds.is_null() {
            None
        } else {
            Some(*unsafe { ref_from_c(bounds as *const LazyBounds, "bounds") }?)
        };
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(
            Variable::new(name, vtype_from_c(vtype), bounds).map_err(CapiError::errored)?,
        );
        Ok(())
    })
}

/// Clones a variable record.
///
/// # Safety
/// `var` must be a valid variable handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_clone(var: *const LmVar, out: *mut *mut LmVar) -> LmStatus {
    guard(|| {
        let var = unsafe { ref_from_c(var as *const Variable, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(var.clone());
        Ok(())
    })
}

/// Compares two variable records.
///
/// # Safety
/// `var` and `other` must be valid handles and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_eq(
    var: *const LmVar,
    other: *const LmVar,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let var = unsafe { ref_from_c(var as *const Variable, "var") }?;
        let other = unsafe { ref_from_c(other as *const Variable, "other") }?;
        *unsafe { out_from_c(out) }? = var == other;
        Ok(())
    })
}

pub use access::{lm_var_bounds, lm_var_inverted, lm_var_name, lm_var_vtype};
