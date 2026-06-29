use std::ffi::{CStr, c_char};

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
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vtypes = cc.vtypes().map(vtype_to_c).collect::<Vec<LmVtype>>();
        *out = vtype_buffer_from_vec(vtypes);
        Ok(())
    })
}

/// Returns the distinct constraint types used by the constraint-collection.
///
/// # Safety
/// `cc` must be a valid constraint-collection handle created by this library.
/// `out` must point to writable `LmCtypeBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_ctypes(cc: *const LmCC, out: *mut LmCmpBuffer) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let ctypes = cc.ctypes().map(cmp_to_c).collect::<Vec<LmComparator>>();
        *out = cmp_buffer_from_vec(ctypes);
        Ok(())
    })
}

/// Returns the distinct variables used by the constraint-collection.
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
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vars = cc
            .vars()
            .map(|v| Box::into_raw(Box::new(v)) as *mut LmVref)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns the distinct variables used by the constraint-collection including inverted binary variables.
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
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vars = cc
            .complete_vars()
            .map(|v| Box::into_raw(Box::new(v)) as *mut LmVref)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns `true` when the constraint-collection contains no constraints.
///
/// # Safety
/// `cc` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_is_empty(cc: *const LmCC, out: *mut bool) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = cc.is_empty();
        Ok(())
    })
}

/// Returns the number of constraints in the collection.
///
/// # Safety
/// `cc` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_len(cc: *const LmCC, out: *mut usize) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = cc.len();
        Ok(())
    })
}

/// Retrieves a constraint by name.
///
/// # Safety
/// `cc` must be a valid handle and `out` must point to writable storage.
/// `key` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmConstraint` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_get(
    cc: *const LmCC,
    key: *const c_char,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;

        if key.is_null() {
            return Err(CapiError::null_pointer("key"));
        }

        let key = unsafe { CStr::from_ptr(key) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`key` must be valid UTF-8: {e}"),
            )
        })?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let constr = cc
            .get(key)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

        *out = Box::into_raw(Box::new(constr)) as *mut LmConstraint;

        Ok(())
    })
}
