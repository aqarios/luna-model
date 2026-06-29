use std::ffi::{CStr, c_char};

use luna_model::core::{Constraint, ConstraintCollection, Expression, prelude::VarRef};

use crate::{
    LmBuffer, LmCC, LmConstraint, LmExpr, LmStatus, LmVref,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
};

/// Inserts a constraint, optionally overriding its current name.
///
/// If `name == NULL`, the collection either keeps the constraint's
/// existing explicit name or synthesizes a fallback name for auto-named
/// constraints. The inserted constraint is renamed to the final collection
/// key so the two stay in sync.
///
/// # Safety
/// `cc` must be a valid mutable constraint-collection handle created by this library.
/// `constr` must be a valid constraint handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_add_constraint(
    cc: *mut LmCC,
    constr: *const LmConstraint,
    name: *const c_char,
    out: *mut LmBuffer,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *mut ConstraintCollection).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let constr = unsafe { (constr as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let name = if name.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(name) }
                    .to_str()
                    .map_err(|e| {
                        CapiError::new(
                            LmStatus::InvalidArgument,
                            format!("`name` must be valid UTF-8: {e}"),
                        )
                    })?
                    .to_owned(),
            )
        };

        let cname = cc
            .add_constraint(constr.clone(), name)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

        *out = buffer_from_vec(cname.into_bytes());
        Ok(())
    })
}

/// Inserts a constraint-collection, optionally prefixing its current constraints' names.
///
/// If `prefix == NULL`, the collection keeps the constraints' existing name.
///
/// # Safety
/// `cc` must be a valid mutable constraint-collection handle created by this library.
/// `other` must be a valid constraint-collection handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_add_collection(
    cc: *mut LmCC,
    other: *const LmCC,
    prefix: *const c_char,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *mut ConstraintCollection).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let other = unsafe { (other as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let prefix = if prefix.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(prefix) }
                    .to_str()
                    .map_err(|e| {
                        CapiError::new(
                            LmStatus::InvalidArgument,
                            format!("`prefix` must be valid UTF-8: {e}"),
                        )
                    })?
                    .to_owned(),
            )
        };
        cc.add_collection(other.clone(), prefix)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        Ok(())
    })
}

/// Replaces an existing constraint while keeping its collection key stable.
///
/// # Safety
/// `cc` must be a valid mutable constraint-collection handle created by this library.
/// `key` must point to a valid NUL-terminated C string.
/// `constr` must be a valid constraint handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_set_constraint(
    cc: *mut LmCC,
    key: *const c_char,
    constr: *const LmConstraint,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *mut ConstraintCollection).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let constr = unsafe { (constr as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;

        if key.is_null() {
            return Err(CapiError::null_pointer("key"));
        }

        let key = unsafe { CStr::from_ptr(key) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`key` must be valid UTF-8: {e}"),
            )
        })?;

        cc.set_constraint(key, constr.clone())
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        Ok(())
    })
}

/// Removes a constraint by name.
///
/// # Safety
/// `cc` must be a valid mutable constraint-collection handle created by this library.
/// `key` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_remove_constraint(cc: *mut LmCC, key: *const c_char) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *mut ConstraintCollection).as_mut() }
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

        cc.remove_constraint(key)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        Ok(())
    })
}

/// Substitute the variable with a given expression.
///
/// # Safety
/// `cc` must be a valid mutable constraint-collection handle created by this library.
/// `target` must be a valid vref handle created by this library.
/// `replacement` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_substitute(
    cc: *mut LmCC,
    target: *const LmVref,
    replacement: *const LmExpr,
) -> LmStatus {
    guard(|| {
        let cc = unsafe { (cc as *mut ConstraintCollection).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        let target = unsafe { (target as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("target"))?;
        let replacement = unsafe { (replacement as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("replacement"))?;

        cc.substitute(target, replacement)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

        Ok(())
    })
}
