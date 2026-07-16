use luna_model::core::prelude::VarRef;

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    expr::utils::{expression_from_c, expression_out_from_c},
    utils::ref_from_c,
};

/// Replaces every occurrence of `target` with `replacement`.
///
/// The substitution is performed term-by-term and preserves the current
/// environment. Any resulting higher-degree terms are rebuilt through the
/// normal expression arithmetic APIs.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `target` must be a valid vref handle created by this library.
/// `replacement` must be a valid expression handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_substitute(
    expr: *const LmExpr,
    target: *const LmVref,
    replacement: *const LmExpr,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { expression_from_c(replacement, "replacement") }?.read();
        let result = expr
            .substitute(target, &replacement)
            .map_err(CapiError::errored)?;
        unsafe { expression_out_from_c(out, result) }
    })
}
