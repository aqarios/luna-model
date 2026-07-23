use lunamodel_core::{ops::LmPow, prelude::VarRef};

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    expr::utils::{expression_from_c, expression_out_from_c},
    utils::{handle_to_c, opt_from_c, out_from_c, ref_from_c},
};

/// Multiplies two variable references, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `other` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_mul_vref(
    vref: *const LmVref,
    other: *const LmVref,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let other = unsafe { ref_from_c(other as *const VarRef, "other") }?;
        let expr = (vref * other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Multiplies a variable reference by a double, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_mul_bias(
    vref: *const LmVref,
    other: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let expr = (vref * other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Adds two variable references, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `other` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_add_vref(
    vref: *const LmVref,
    other: *const LmVref,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let other = unsafe { ref_from_c(other as *const VarRef, "other") }?;
        let expr = (vref + other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Adds a double value to a variable reference, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_add_bias(
    vref: *const LmVref,
    other: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let expr = (vref + other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Subtracts two variable references, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `other` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_sub_vref(
    vref: *const LmVref,
    other: *const LmVref,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let other = unsafe { ref_from_c(other as *const VarRef, "other") }?;
        let expr = (vref - other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Subtracts an expression from a variable references, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_sub_expr(
    vref: *const LmVref,
    expr: *const LmExpr,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let other = unsafe { expression_from_c(expr, "other") }?.read();
        let expr = (vref - &*other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Subtract a double value to a variable reference, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_sub_bias(
    vref: *const LmVref,
    other: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let expr = (vref - other).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Subtracts a variable reference from a double value (`other - vref`),
/// promoting the result to an expression (Rust `bias - vref`).
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_rsub_bias(
    vref: *const LmVref,
    other: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let expr = (other - vref.clone()).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Negate a variable reference, promoting the result to an expression
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_neg(vref: *const LmVref, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let expr = (-vref).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Invert a variable reference, resulting in a reference to the inverted variable.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_not(vref: *const LmVref, out: *mut *mut LmVref) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        let vref = (!vref).map_err(CapiError::errored)?;
        *out = handle_to_c(vref);
        Ok(())
    })
}

/// Raise a variable reference to a power, promoting the result to an expression.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_pow(
    vref: *const LmVref,
    n: usize,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let expr = (vref.pow(n)).map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, expr) }
    })
}

/// Evaluates if the value satisfies this vref's current bounds.
///
/// # Safety
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_eval(
    vref: *const LmVref,
    value: f64,
    tol: *const f64,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        let tol = unsafe { opt_from_c(tol) };
        *out = (vref.evaluate(value, tol)).map_err(CapiError::errored)?;
        Ok(())
    })
}
