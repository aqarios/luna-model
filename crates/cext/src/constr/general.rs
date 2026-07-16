use luna_model::core::prelude::{ContentEquality, VarRef};

use std::ffi::c_char;

use crate::{
    LmConstraint, LmEnv, LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    eval::NameSample,
    expr::utils::expression_from_c,
    utils::{
        new_handle_out_from_c, opt_from_c, out_from_c, ref_from_c, shared_from_c, slice_from_c,
        storage_from_c,
    },
};

/// Evaluates a constraint against a name-keyed sample.
///
/// # Safety
/// `constr` must be valid, `names` and `values` must reference `len` readable
/// elements, `tol` may be null or readable, and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_evaluate_sample(
    constr: *const LmConstraint,
    names: *const *const c_char,
    values: *const f64,
    len: usize,
    tol: *const f64,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { shared_from_c(constr, "constr") }?.read_arc();
        let sample = unsafe { NameSample::from_c(names, values, len) }?;
        let out = unsafe { out_from_c(out) }?;
        *out = constr
            .evaluate_sample(&sample, unsafe { opt_from_c(tol) })
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Clones a constraint into a new, independent handle written to `out`.
///
/// # Safety
/// `out` must point to writable `LmConstraint *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_clone(
    constr: *const LmConstraint,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { shared_from_c(constr, "constr") }?
            .read_arc()
            .clone();
        unsafe { new_handle_out_from_c(out, constr) }
    })
}

/// Deep-Clones a constraint into a new, independent handle written to `out`.
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmConstraint *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_deep_clone(
    constr: *const LmConstraint,
    env: *const LmEnv,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { shared_from_c(constr, "constr") }?.read_arc();
        let env = unsafe { storage_from_c(env, "env")? };
        unsafe { new_handle_out_from_c(out, constr.deep_clone(env.clone())) }
    })
}

/// Substitutes every occurrence of `target` in the constraint with `replacement`.
///
/// # Safety
/// `constr` must be a valid mutable constraint handle created by this library.
/// `target` must be a valid vref handle created by this library.
/// `replacement` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_substitute(
    constr: *mut LmConstraint,
    target: *const LmVref,
    replacement: *const LmExpr,
) -> LmStatus {
    guard(|| {
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { expression_from_c(replacement, "replacement") }?
            .read()
            .clone();
        let mut constr = unsafe { shared_from_c(constr, "constr") }?.write_arc();

        constr
            .substitute(target, &replacement)
            .map_err(CapiError::errored)?;

        Ok(())
    })
}

/// Compares two constraints for equal contents (ignoring different environments).
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `other` must be a valid constraint handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_equal_contents(
    constr: *const LmConstraint,
    other: *const LmConstraint,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { shared_from_c(constr, "constr") }?.read_arc();
        let other = unsafe { shared_from_c(other, "other") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = constr.equal_contents(&other);
        Ok(())
    })
}

/// Compares two constraints for equality (ignoring different environments).
///
/// # Safety
/// `constr` must be a valid constraint handle created by this library.
/// `other` must be a valid constraint handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_eq(
    constr: *const LmConstraint,
    other: *const LmConstraint,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { shared_from_c(constr, "constr") }?.read_arc();
        let other = unsafe { shared_from_c(other, "other") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = constr.eq(&other);
        Ok(())
    })
}

/// Evaluates a constraint against an index-aligned dense lookup.
///
/// # Safety
/// `constr` must be a valid constraint handle, `lookup` must reference `len`
/// readable doubles, `tol` may be null or readable, and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_evaluate_sample_quick(
    constr: *const LmConstraint,
    lookup: *const f64,
    len: usize,
    tol: *const f64,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { shared_from_c(constr, "constr") }?.read_arc();
        let lookup = unsafe { slice_from_c(lookup, len, "lookup") }?;
        let tol = unsafe { opt_from_c(tol) };
        let out = unsafe { out_from_c(out) }?;
        *out = constr
            .evaluate_sample_quick(lookup, tol)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}
