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
/// `constraint` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_clone(
    constr: *const LmConstraint,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { (constr as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(constr.clone())) as *mut LmConstraint;
        Ok(())
    })
}

/// Deep-Clones a constraint into a new, independent handle written to `out`.
///
/// # Safety
/// `constr` and `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_constr_deep_clone(
    constr: *const LmConstraint,
    env: *const LmEnv,
    out: *mut *mut LmConstraint,
) -> LmStatus {
    guard(|| {
        let constr = unsafe { (constr as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;
        let env = unsafe { environment_from_c(env)? };
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(constr.deep_clone(env.clone()))) as *mut LmConstraint;
        Ok(())
    })
}

/// Substitute the variable with a given expression.
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
        let constr = unsafe { (constr as *mut Constraint).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;
        let target = unsafe { (target as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("target"))?;
        let replacement = unsafe { (replacement as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("replacement"))?;

        constr
            .substitute(target, replacement)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

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
        let constr = unsafe { (constr as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;
        let other = unsafe { (other as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
        let constr = unsafe { (constr as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("constr"))?;
        let other = unsafe { (other as *const Constraint).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = constr.eq(other);
        Ok(())
    })
}
