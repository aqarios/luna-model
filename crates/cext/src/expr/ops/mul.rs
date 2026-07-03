use std::ops::Mul;

use luna_model::core::{Expression, ops::LmMulAssign, prelude::VarRef};

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    utils::{handle_to_c, mut_from_c, out_from_c, ref_from_c},
};

// //////////// //
// MUL IN PLACE //
// //////////// //

/// Multiplies a variable reference to the expression in place.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `vref` must be a valid vref handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_imul_vref(expr: *mut LmExpr, vref: *const LmVref) -> LmStatus {
    guard(|| {
        let expr = unsafe { mut_from_c(expr as *mut Expression, "expr") }?;
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        expr.mul_assign(vref).map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Multiplies a bias to the expression in place.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_imul_bias(expr: *mut LmExpr, bias: f64) -> LmStatus {
    guard(|| {
        let expr = unsafe { mut_from_c(expr as *mut Expression, "expr") }?;
        expr.mul_assign(bias).map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Multiplies an expression reference to the expression in place.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_imul_expr(expr: *mut LmExpr, other: *const LmExpr) -> LmStatus {
    guard(|| {
        let expr = unsafe { mut_from_c(expr as *mut Expression, "expr") }?;
        let other = unsafe { ref_from_c(other as *const Expression, "other") }?;
        expr.mul_assign(other).map_err(CapiError::errored)?;
        Ok(())
    })
}

// ///////////////// //
// mul RESULT IN NEW //
// ///////////////// //

/// Multiplies a variable reference to the expression, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_mul_vref(
    expr: *const LmExpr,
    vref: *const LmVref,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let out = unsafe { out_from_c(out) }?;
        let result = expr.clone().mul(vref).map_err(CapiError::errored)?;
        *out = handle_to_c(result);
        Ok(())
    })
}

/// Multiplies a bias to the expression in place, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_mul_bias(
    expr: *const LmExpr,
    bias: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let out = unsafe { out_from_c(out) }?;
        let result = expr.clone().mul(bias).map_err(CapiError::errored)?;
        *out = handle_to_c(result);
        Ok(())
    })
}

/// Multiplies an expression reference to the expression in place, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_mul_expr(
    expr: *const LmExpr,
    other: *const LmExpr,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let other = unsafe { ref_from_c(other as *const Expression, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        let result = expr.clone().mul(other).map_err(CapiError::errored)?;
        *out = handle_to_c(result);
        Ok(())
    })
}
