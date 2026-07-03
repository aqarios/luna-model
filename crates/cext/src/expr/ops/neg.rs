use luna_model::core::Expression;

use crate::{
    LmExpr, LmStatus,
    error::guard,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Negate an expression reference.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_neg(expr: *const LmExpr, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(-expr);
        Ok(())
    })
}
