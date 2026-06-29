use luna_model::core::{Expression, prelude::ContentEquality};

use crate::{
    LmExpr, LmStatus,
    error::{CapiError, guard},
};

/// Compares two expressions for equality.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_eq(
    expr: *const LmExpr,
    other: *const LmExpr,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let other = unsafe { (other as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.eq(other);
        Ok(())
    })
}

/// Compares two expressions for equal contents (ignoring different environments).
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `other` must be a valid expression handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_equal_contents(
    expr: *const LmExpr,
    other: *const LmExpr,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let other = unsafe { (other as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.equal_contents(other);
        Ok(())
    })
}
