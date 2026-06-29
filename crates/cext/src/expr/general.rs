use luna_model::core::Expression;

use crate::{
    LmEnv, LmExpr, LmStatus,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

/// Clones an expression into a new, independent handle written to `out`.
///
/// # Safety
/// `expr` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_clone(expr: *const LmExpr, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(expr.clone())) as *mut LmExpr;
        Ok(())
    })
}

/// Deep Clones an expression into a new, independent handle written to `out` for a new environment
/// `env`
///
/// # Safety
/// `expr` and `env` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_deep_clone(
    expr: *const LmExpr,
    env: *const LmEnv,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let env = unsafe { environment_from_c(env)? };
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(expr.deep_clone(env.clone()))) as *mut LmExpr;
        Ok(())
    })
}
