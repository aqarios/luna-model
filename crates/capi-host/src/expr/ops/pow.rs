use lunamodel_core::ops::LmPow;

use crate::{
    LmExpr, LmStatus,
    error::{CapiError, guard},
    expr::utils::{expression_from_c, expression_out_from_c},
};

/// Raises the expression to a non-negative power in-place.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_ipow(expr: *mut LmExpr, n: usize) -> LmStatus {
    guard(|| {
        let mut expr = unsafe { expression_from_c(expr, "expr") }?.write();
        expr.pow_assign(n).map_err(CapiError::from)?;
        Ok(())
    })
}

/// Raises the expression to a non-negative power, resulting in a new expression.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_pow(
    expr: *const LmExpr,
    n: usize,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let result = expr.pow(n).map_err(CapiError::from)?;
        unsafe { expression_out_from_c(out, result) }
    })
}
