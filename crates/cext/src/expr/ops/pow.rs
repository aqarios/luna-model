use luna_model::core::{
    Expression,
    ops::{LmPow, LmPowAssign},
};

use crate::{
    LmExpr, LmStatus,
    error::{CapiError, guard},
    utils::{handle_to_c, mut_from_c, out_from_c, ref_from_c},
};

/// Raises the expression to a non-negative power in-place.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_ipow(expr: *mut LmExpr, n: usize) -> LmStatus {
    guard(|| {
        let expr = unsafe { mut_from_c(expr as *mut Expression, "expr") }?;
        expr.pow_assign(n).map_err(CapiError::errored)?;
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
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(expr.pow(n));
        Ok(())
    })
}
