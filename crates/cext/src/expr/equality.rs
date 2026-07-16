use luna_model::core::prelude::ContentEquality;

use super::utils::expression_from_c;
use crate::{LmExpr, LmStatus, error::guard, utils::out_from_c};

/// Compares two expressions for equality.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_eq(
    expr: *const LmExpr,
    other: *const LmExpr,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let other = unsafe { expression_from_c(other, "other") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.eq(&other);
        Ok(())
    })
}

/// Compares two expressions for equal contents (ignoring different environments).
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_equal_contents(
    expr: *const LmExpr,
    other: *const LmExpr,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let other = unsafe { expression_from_c(other, "other") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.equal_contents(&other);
        Ok(())
    })
}
