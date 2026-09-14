use std::ops::Sub;

use lunamodel_core::{ops::LmSubAssign, prelude::VarRef};

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    expr::utils::{expression_from_c, expression_out_from_c},
    utils::ref_from_c,
};

// //////////// //
// SUB IN PLACE //
// //////////// //

/// Subs a variable reference from the expression in place.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `vref` must be a valid vref handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_isub_vref(expr: *mut LmExpr, vref: *const LmVref) -> LmStatus {
    guard(|| {
        let mut expr = unsafe { expression_from_c(expr, "expr") }?.write();
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        expr.sub_assign(vref).map_err(CapiError::from)?;
        Ok(())
    })
}

/// Subs a bias from the expression in place.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_isub_bias(expr: *mut LmExpr, bias: f64) -> LmStatus {
    guard(|| {
        let mut expr = unsafe { expression_from_c(expr, "expr") }?.write();
        expr.sub_assign(bias).map_err(CapiError::from)?;
        Ok(())
    })
}

/// Subs an expression reference from the expression in place.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_isub_expr(expr: *mut LmExpr, other: *const LmExpr) -> LmStatus {
    guard(|| {
        let other = unsafe { expression_from_c(other, "other") }?.read().clone();
        let mut expr = unsafe { expression_from_c(expr, "expr") }?.write();
        expr.sub_assign(&other).map_err(CapiError::from)?;
        Ok(())
    })
}

// ///////////////// //
// SUB RESULT IN NEW //
// ///////////////// //

/// Subs a variable reference from the expression, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `vref` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_sub_vref(
    expr: *const LmExpr,
    vref: *const LmVref,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let vref = unsafe { ref_from_c(vref as *const VarRef, "vref") }?;
        let result = expr.clone().sub(vref).map_err(CapiError::from)?;
        unsafe { expression_out_from_c(out, result) }
    })
}

/// Subs a bias from the expression in place, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_sub_bias(
    expr: *const LmExpr,
    bias: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let result = expr.clone().sub(bias).map_err(CapiError::from)?;
        unsafe { expression_out_from_c(out, result) }
    })
}

// Subs an expression reference from the expression in place, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_sub_expr(
    expr: *const LmExpr,
    other: *const LmExpr,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read().clone();
        let other = unsafe { expression_from_c(other, "other") }?.read().clone();
        let result = expr.sub(&other).map_err(CapiError::from)?;
        unsafe { expression_out_from_c(out, result) }
    })
}

// ///////////////// //
// R-SUB RESULT IN NEW //
// ///////////////// //

/// Subs a bias from the expression in place, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_rsub_bias(
    expr: *const LmExpr,
    bias: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let result = (bias - &*expr).map_err(CapiError::from)?;
        unsafe { expression_out_from_c(out, result) }
    })
}
