use lunamodel_core::prelude::Variable;

use crate::{
    LmBounds, LmBuffer, LmStatus, LmVar, LmVtype,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::vtype_to_c,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Returns a copy of the variable's name as an owned UTF-8 byte buffer.
///
/// The returned buffer is written to `out` and must be released with
/// `lm_buffer_free`.
///
/// The buffer is not guaranteed to be NUL-terminated. C callers should use
/// `buffer.data` together with `buffer.len`.
///
/// # Safety
/// `var` must be a valid variable handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_name(var: *const LmVar, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let var = unsafe { ref_from_c(var as *const Variable, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        let name = var.name();
        *out = buffer_from_vec(name.as_bytes().to_vec());
        Ok(())
    })
}

/// Returns a copy of the variable's vtype.
///
/// # Safety
/// `var` must be a valid variable handle created by this library.
/// `out` must point to writable `LmVtype` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_vtype(var: *const LmVar, out: *mut LmVtype) -> LmStatus {
    guard(|| {
        let var = unsafe { ref_from_c(var as *const Variable, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        let vtype = var.vtype();
        *out = vtype_to_c(vtype);
        Ok(())
    })
}

/// Returns the variable's bounds.
///
/// # Safety
/// `var` must be a valid variable handle created by this library.
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_bounds(var: *const LmVar, out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let var = unsafe { ref_from_c(var as *const Variable, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        let bounds = *var.bounds();
        *out = handle_to_c(bounds);
        Ok(())
    })
}

/// Returns the variable id of the inverted companion, if one exists.
///
/// On success (`LmStatus::Ok`), `*out` is set to the companion's variable id.
/// If the variable has no inverted companion, returns `LmStatus::Absent` and
/// leaves `*out` unchanged.
///
/// # Safety
/// `var` must be a valid variable handle created by this library.
/// `out` must point to writable `uint32_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_inverted(var: *const LmVar, out: *mut u32) -> LmStatus {
    guard(|| {
        let var = unsafe { ref_from_c(var as *const Variable, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        match var.inverted {
            Some(id) => {
                *out = id;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "variable has no inverted companion",
            )),
        }
    })
}
