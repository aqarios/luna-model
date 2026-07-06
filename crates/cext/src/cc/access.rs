use crate::utils::{handle_to_c, out_from_c, ref_from_c, str_from_c};
use std::ffi::c_char;

use luna_model::core::ConstraintCollection;

use crate::{
    LmCC, LmCmpBuffer, LmComparator, LmConstraint, LmStatus, LmVref, LmVrefBuffer, LmVtype,
    LmVtypeBuffer,
    buffer::{cmp_buffer_from_vec, vref_buffer_from_vec, vtype_buffer_from_vec},
    error::{CapiError, guard},
    utils::{cmp_to_c, vtype_to_c},
};

/// Returns the distinct variable types used by the constraint-collection.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmVtypeBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_vtypes(cc: *const LmCC, out: *mut LmVtypeBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        let vtypes = cc.vtypes().map(vtype_to_c).collect::<Vec<LmVtype>>();
        *out = vtype_buffer_from_vec(vtypes);
        Ok(())
    })
}

/// Returns the distinct constraint types used by the constraint-collection.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmCmpBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_ctypes(cc: *const LmCC, out: *mut LmCmpBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        let ctypes = cc.ctypes().map(cmp_to_c).collect::<Vec<LmComparator>>();
        *out = cmp_buffer_from_vec(ctypes);
        Ok(())
    })
}

/// Returns the distinct variables used by the constraint-collection including inverted binary variables.
///
/// This matches `ConstraintCollection::complete_vars()` in the Rust API.
/// The returned array is owned by the caller and must be released with `lm_vref_buffer_free`.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmVrefBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_complete_vars(cc: *const LmCC, out: *mut LmVrefBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        let vars = cc
            .complete_vars()
            .map(handle_to_c)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns the distinct variables used by the constraint-collection.
///
/// This matches `ConstraintCollection::vars()` in the Rust API.
/// The returned array is owned by the caller and must be released with `lm_vref_buffer_free`.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmVrefBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_vars(cc: *const LmCC, out: *mut LmVrefBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        let vars = cc.vars().map(handle_to_c).collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns `true` when the constraint-collection contains no constraints.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_is_empty(cc: *const LmCC, out: *mut bool) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = cc.is_empty();
        Ok(())
    })
}

/// Returns the number of constraints in the collection.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_len(cc: *const LmCC, out: *mut usize) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = cc.len();
        Ok(())
    })
}

/// Retrieves a constraint by name.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmConstraint *` storage.
/// `key` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmConstraint *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_get(
    cc: *const LmCC,
    key: *const c_char,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;

        let key = unsafe { str_from_c(key, "key") }?;

        let out = unsafe { out_from_c(out) }?;

        let constr = cc.get(key).map_err(CapiError::errored)?;

        *out = handle_to_c(constr);

        Ok(())
    })
}
