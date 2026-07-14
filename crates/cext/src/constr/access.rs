use crate::utils::{handle_to_c, mut_from_c, out_from_c, ref_from_c, str_from_c};
use std::ffi::c_char;

use luna_model::core::prelude::Constraint;

use crate::{
    LmBuffer, LmComparator, LmConstraint, LmExpr, LmStatus,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::cmp_to_c,
};

/// Returns a copy of the constraint's name as an owned UTF-8 byte buffer.
///
/// The returned buffer is written to `out` and must be released with
/// `lm_buffer_free`.
///
/// The buffer is not guaranteed to be NUL-terminated. C callers should use
/// `buffer.data` together with `buffer.len`.
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_name(
    constr: *const LmConstraint,
    out: *mut LmBuffer,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;
        let name = constr.name();
        *out = buffer_from_vec(name.as_bytes().to_vec());
        Ok(())
    })
}

/// Returns a copy of the constraint's lhs.
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_lhs(
    constr: *const LmConstraint,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(constr.lhs.clone());
        Ok(())
    })
}

/// Returns a copy of the constraint's rhs.
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_rhs(constr: *const LmConstraint, out: *mut f64) -> LmStatus {
    guard(|| {
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = constr.rhs;
        Ok(())
    })
}

/// Returns a copy of the constraint's comparator.
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `out` must point to writable `LmComparator` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_comparator(
    constr: *const LmConstraint,
    out: *mut LmComparator,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = cmp_to_c(constr.comparator);
        Ok(())
    })
}

/// Sets the constraint name.
///
/// `name` must be non-null and valid UTF-8.
///
/// # Safety
/// `constr` must be a valid mutable constraint handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_set_name(
    constr: *mut LmConstraint,
    name: *const c_char,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { mut_from_c(constr as *mut Constraint, "constr") }?;

        let name = unsafe { str_from_c(name, "name") }?;
        constr
            .set_name(name.to_owned())
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Returns whether the constraint received an automatically generated name.
///
/// # Safety
/// `constr` must be a valid constraint handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_auto_name(
    constr: *const LmConstraint,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = constr.auto_name;
        Ok(())
    })
}
