use crate::utils::{handle_to_c, out_from_c, ref_from_c};
use luna_model::core::Expression;

use crate::{LmEnv, LmExpr, LmStatus, env::utils::environment_from_c, error::guard};

/// Clones an expression into a new, independent handle written to `out`.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_clone(expr: *const LmExpr, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(expr.clone());
        Ok(())
    })
}

/// Deep-clones an expression into a new, independent handle written to `out`,
/// rebinding it to the environment `env`.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_deep_clone(
    expr: *const LmExpr,
    env: *const LmEnv,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let env = unsafe { environment_from_c(env)? };
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(expr.deep_clone(env.clone()));
        Ok(())
    })
}
