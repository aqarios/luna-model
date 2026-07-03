use luna_model::core::{Expression, prelude::VarRef};

use crate::{
    LmExpr, LmStatus, LmVref,
    error::{CapiError, guard},
    utils::{handle_to_c, out_from_c, ref_from_c, slice_from_c},
};

/// Splits an expression into terms that do and do not touch the given variables.
///
/// The first returned expression contains every contribution involving at
/// least one variable from `vrefs`. The second contains the remaining
/// contributions.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `vars` must point to `vars_len` readable `LmVref *` handles created by this library.
/// `first` and `second` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_separate(
    expr: *const LmExpr,
    vars: *const *const LmVref,
    vars_len: usize,
    first: *mut *mut LmExpr,
    second: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let vars = unsafe { slice_from_c(vars, vars_len, "vars") }?;
        let vrefs = vars
            .iter()
            .map(|&var| unsafe { ref_from_c(var as *const VarRef, "vars") }.cloned())
            .collect::<Result<Vec<_>, _>>()?;
        let first = unsafe { out_from_c(first) }?;
        let second = unsafe { out_from_c(second) }?;
        let (first_expr, second_expr) = expr.separate(&vrefs).map_err(CapiError::errored)?;
        *first = handle_to_c(first_expr);
        *second = handle_to_c(second_expr);
        Ok(())
    })
}
