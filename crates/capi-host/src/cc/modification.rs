use std::ffi::c_char;

use lunamodel_core::prelude::VarRef;

use super::utils::collection_from_c;
use crate::{
    LmBuffer, LmCC, LmConstraint, LmExpr, LmStatus, LmStrBuffer, LmVref,
    buffer::{buffer_from_vec, str_buffer_from_vec},
    error::{CapiError, guard},
    expr::utils::expression_from_c,
    utils::{
        opt_slice_from_c, opt_str_from_c, out_from_c, ref_from_c, shared_from_c, slice_from_c,
        str_from_c,
    },
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
        let constr = unsafe { shared_from_c(constr, "constr") }?
            .read_arc()
            .clone();
        let out = unsafe { out_from_c(out) }?;

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);

        let mut cc = unsafe { collection_from_c(cc, "cc") }?.write();
        let cname = cc
            .add_constraint(constr, name)
            .map_err(CapiError::errored)?;

        *out = buffer_from_vec(cname.into_bytes());
        Ok(())
    })
}

/// Inserts `len` constraints in one call, optionally overriding each one's name.
///
/// `names` may be null to mean no explicit name for any constraint; when non-null, it must point
/// to `len` (possibly individually null) C strings, each following the same "keep existing/
/// synthesize a fallback" rule as [`lm_cc_add_constraint`]'s `name`. Writes the final assigned
/// names, in the same order as `constrs`, to `out`.
///
/// # Safety
/// `cc` must be a valid mutable constraint-collection handle created by this library.
/// `constrs` must point to `len` valid constraint handles created by this library.
/// `names`, if non-null, must point to `len` C strings, each null or NUL-terminated.
/// `out` must point to writable `LmStrBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_add_many_constraints(
    cc: *mut LmCC,
    constrs: *const *const LmConstraint,
    names: *const *const c_char,
    len: usize,
    out: *mut LmStrBuffer,
) -> LmStatus {
    guard(|| {
        let constrs = unsafe { slice_from_c(constrs, len, "constrs") }?
            .iter()
            .map(|&c| unsafe { shared_from_c(c, "constr") }.map(|c| c.read_arc().clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let names = match unsafe { opt_slice_from_c(names, len) } {
            None => vec![None; constrs.len()],
            Some(names) => names
                .iter()
                .map(|&n| unsafe { opt_str_from_c(n, "name") }.map(|s| s.map(str::to_owned)))
                .collect::<Result<Vec<_>, _>>()?,
        };

        let out = unsafe { out_from_c(out) }?;
        let mut cc = unsafe { collection_from_c(cc, "cc") }?.write();
        let assigned = cc
            .add_many_constraints(constrs.into_iter().zip(names))
            .map_err(CapiError::errored)?;
        *out = str_buffer_from_vec(assigned);
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
    out: *mut LmStrBuffer,
) -> LmStatus {
    guard(|| {
        let other = unsafe { collection_from_c(other, "other") }?.read().clone();
        let prefix = unsafe { opt_str_from_c(prefix, "prefix") }?.map(str::to_owned);
        let out = unsafe { out_from_c(out) }?;
        let mut cc = unsafe { collection_from_c(cc, "cc") }?.write();
        *out = str_buffer_from_vec(
            cc.add_collection(other, prefix)
                .map_err(CapiError::errored)?,
        );
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
        let constr = unsafe { shared_from_c(constr, "constr") }?
            .read_arc()
            .clone();

        let key = unsafe { str_from_c(key, "key") }?;

        let mut cc = unsafe { collection_from_c(cc, "cc") }?.write();
        cc.set_constraint(key, constr).map_err(CapiError::errored)?;
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
        let key = unsafe { str_from_c(key, "key") }?;
        let mut cc = unsafe { collection_from_c(cc, "cc") }?.write();
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
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { expression_from_c(replacement, "replacement") }?
            .read()
            .clone();
        let mut cc = unsafe { collection_from_c(cc, "cc") }?.write();

        cc.substitute(target, &replacement)
            .map_err(CapiError::errored)?;

        Ok(())
    })
}
