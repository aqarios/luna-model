use luna_model::core::Expression;

use crate::{
    LmExpr, LmStatus,
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
