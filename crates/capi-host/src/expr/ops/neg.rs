use crate::{
    LmExpr, LmStatus,
    error::guard,
    expr::utils::{expression_from_c, expression_out_from_c},
};

/// Negate an expression reference.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_neg(expr: *const LmExpr, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        unsafe { expression_out_from_c(out, -&*expr) }
    })
}
