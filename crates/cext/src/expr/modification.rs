use crate::{LmExpr, LmStatus, error::guard, expr::utils::expression_from_c, utils::slice_from_c};

/// Replaces an expression's constant offset.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_set_offset(expr: *mut LmExpr, offset: f64) -> LmStatus {
    guard(|| {
        unsafe { expression_from_c(expr, "expr") }?.write().offset = offset;
        Ok(())
    })
}

/// Adds one raw linear contribution to an expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_add_linear_term(
    expr: *mut LmExpr,
    variable: u32,
    bias: f64,
) -> LmStatus {
    guard(|| {
        unsafe { expression_from_c(expr, "expr") }?.write().linear += (variable, bias);
        Ok(())
    })
}

/// Adds one raw quadratic contribution to an expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_add_quadratic_term(
    expr: *mut LmExpr,
    u: u32,
    v: u32,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let expression = unsafe { expression_from_c(expr, "expr") }?;
        let mut expression = expression.write();
        *expression.quadratic.get_or_insert_default() += (u, v, bias);
        Ok(())
    })
}

/// Adds one raw higher-order contribution to an expression.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `variables` must be null when `len` is zero or point to `len` readable
/// variable indices.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_add_higher_order_term(
    expr: *mut LmExpr,
    variables: *const u32,
    len: usize,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let variables = unsafe { slice_from_c(variables, len, "variables") }?;
        let expression = unsafe { expression_from_c(expr, "expr") }?;
        let mut expression = expression.write();
        *expression.higher_order.get_or_insert_default() += (variables, bias);
        Ok(())
    })
}
