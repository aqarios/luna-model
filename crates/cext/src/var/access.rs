use luna_model::core::prelude::Variable;

use crate::{
    LmBounds, LmBuffer, LmStatus, LmVar, LmVtype,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::vtype_to_c,
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
        let var = unsafe { (var as *const Variable).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("var"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
        let var = unsafe { (var as *const Variable).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("var"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vtype = var.vtype();
        *out = vtype_to_c(vtype);
        Ok(())
    })
}

/// Returns the variable's bounds.
///
/// # Safety
/// `var` must be a valid variable handle created by this library.
/// `out` must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_bounds(var: *const LmVar, out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let var = unsafe { (var as *const Variable).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("var"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let bounds = var.bounds();
        *out = Box::into_raw(Box::new(bounds)) as *mut LmBounds;
        Ok(())
    })
}

/// Returns the variable id of the inverted companion, if one exists.
///
/// On success, when the variable has an inverted companion `*present` is set to
/// `true` and `*out` to the companion's variable id. Otherwise `*present` is set
/// to `false` and `*out` is left unchanged.
///
/// # Safety
/// `var` must be a valid variable handle created by this library.
/// `out` must point to writable `u32` storage.
/// `present` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_inverted(
    var: *const LmVar,
    out: *mut u32,
    present: *mut bool,
) -> LmStatus {
    guard(|| {
        let var = unsafe { (var as *const Variable).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("var"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let present =
            unsafe { present.as_mut() }.ok_or_else(|| CapiError::null_pointer("present"))?;
        match var.inverted {
            Some(id) => {
                *out = id;
                *present = true;
            }
            None => *present = false,
        }
        Ok(())
    })
}
