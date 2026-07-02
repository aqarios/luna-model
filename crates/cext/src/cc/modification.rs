use crate::utils::{mut_from_c, opt_str_from_c, out_from_c, ref_from_c, str_from_c};
use std::ffi::c_char;

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
        let cc = unsafe { mut_from_c(cc as *mut ConstraintCollection, "cc") }?;
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);

        let cname = cc
            .add_constraint(constr.clone(), name)
            .map_err(CapiError::errored)?;

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
        let cc = unsafe { mut_from_c(cc as *mut ConstraintCollection, "cc") }?;
        let other = unsafe { ref_from_c(other as *const ConstraintCollection, "other") }?;
        let prefix = unsafe { opt_str_from_c(prefix, "prefix") }?.map(str::to_owned);
        cc.add_collection(other.clone(), prefix)
            .map_err(CapiError::errored)?;
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
        let cc = unsafe { mut_from_c(cc as *mut ConstraintCollection, "cc") }?;
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;

        let key = unsafe { str_from_c(key, "key") }?;

        cc.set_constraint(key, constr.clone())
            .map_err(CapiError::errored)?;
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
        let cc = unsafe { mut_from_c(cc as *mut ConstraintCollection, "cc") }?;

        let key = unsafe { str_from_c(key, "key") }?;

        cc.remove_constraint(key).map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Substitutes every occurrence of `target` in the collection with `replacement`.
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
        let cc = unsafe { mut_from_c(cc as *mut ConstraintCollection, "cc") }?;
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { ref_from_c(replacement as *const Expression, "replacement") }?;

        cc.substitute(target, replacement)
            .map_err(CapiError::errored)?;

        Ok(())
    })
}
