use crate::utils::{handle_to_c, mut_from_c, opt_from_c, out_from_c, ref_from_c, slice_from_c};
use luna_model::core::{
    Expression,
    prelude::{Constraint, ContentEquality, VarRef},
};

use crate::{
    LmConstraint, LmEnv, LmExpr, LmStatus, LmVref,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

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
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(constr.clone());
        Ok(())
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
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let env = unsafe { environment_from_c(env)? };
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(constr.deep_clone(env.clone()));
        Ok(())
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
        let constr = unsafe { mut_from_c(constr as *mut Constraint, "constr") }?;
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { ref_from_c(replacement as *const Expression, "replacement") }?;

        constr
            .substitute(target, replacement)
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
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let other = unsafe { ref_from_c(other as *const Constraint, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = constr.equal_contents(other);
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
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let other = unsafe { ref_from_c(other as *const Constraint, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = constr.eq(other);
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
        let constr = unsafe { ref_from_c(constr as *const Constraint, "constr") }?;
        let lookup = unsafe { slice_from_c(lookup, len, "lookup") }?;
        let tol = unsafe { opt_from_c(tol) };
        let out = unsafe { out_from_c(out) }?;
        *out = constr
            .evaluate_sample_quick(lookup, tol)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}
