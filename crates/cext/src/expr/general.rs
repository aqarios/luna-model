use crate::utils::{handle_to_c, mut_slice_from_c, out_from_c, ref_from_c, slice_from_c};
use luna_model::core::Expression;

use crate::{
    LmEnv, LmExpr, LmStatus,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

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

/// Deep-clones expressions into one newly cloned shared environment.
///
/// `out` is caller-provided storage for exactly `len` owned expression
/// handles. Each returned handle must be released with [`lm_expr_free`].
///
/// # Safety
/// `exprs` must reference `len` valid expression handles and `out` must
/// reference `len` writable handle slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_deep_clone_many(
    exprs: *const *const LmExpr,
    len: usize,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let exprs = unsafe { slice_from_c(exprs, len, "exprs") }?;
        let exprs = exprs
            .iter()
            .map(|expr| unsafe { ref_from_c(*expr as *const Expression, "expr") })
            .collect::<Result<Vec<_>, _>>()?;
        let cloned = Expression::deep_clone_many(&exprs).map_err(CapiError::errored)?;
        let out = unsafe { mut_slice_from_c(out, len, "out") }?;
        for (slot, expr) in out.iter_mut().zip(cloned) {
            *slot = handle_to_c(expr);
        }
        Ok(())
    })
}

/// Evaluates an expression against an index-aligned dense lookup.
///
/// # Safety
/// `expr` must be a valid expression handle, `lookup` must reference `len`
/// readable doubles, and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_evaluate_sample_quick(
    expr: *const LmExpr,
    lookup: *const f64,
    len: usize,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let lookup = unsafe { slice_from_c(lookup, len, "lookup") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = expr
            .evaluate_sample_quick(lookup)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}
