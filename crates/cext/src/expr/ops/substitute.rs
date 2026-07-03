use luna_model::core::{Expression, prelude::VarRef};

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    utils::{handle_to_c, out_from_c, ref_from_c},
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
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { ref_from_c(replacement as *const Expression, "replacement") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(
            expr.substitute(target, replacement)
                .map_err(CapiError::errored)?,
        );
        Ok(())
    })
}
