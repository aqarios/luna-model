use lunamodel_core::prelude::VarRef;

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    expr::utils::{expression_from_c, expression_out_from_c},
    utils::ref_from_c,
};

/// Computes the partial derivative of `expr` with respect to `var`.
///
/// The input expression is left unchanged. The result is a new expression in
/// the same environment, written to `out`. Fails if `var` belongs to another
/// environment or is no longer in it.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `var` must be a valid vref handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_derivative(
    expr: *const LmExpr,
    var: *const LmVref,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let var = unsafe { ref_from_c(var as *const VarRef, "var") }?;
        let result = expr.derivative(var).map_err(CapiError::from)?;
        unsafe { expression_out_from_c(out, result) }
    })
}
