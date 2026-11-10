use lunamodel_core::prelude::{HigherOrder, Linear, Quadratic};

use crate::{
    LmExpr, LmHigherOrder, LmLinear, LmQuadratic, LmStatus,
    error::{CapiError, guard},
    expr::utils::expression_from_c,
    utils::{ref_from_c, slice_from_c},
};

/// Replaces an expression's constant offset.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_set_offset(expr: *mut LmExpr, offset: f64) -> LmStatus {
    guard(|| {
        unsafe { expression_from_c(expr, "expr") }?
            .write()
            .set_offset(offset);
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
        unsafe { expression_from_c(expr, "expr") }?
            .write()
            .add_linear_term(variable, bias)
            .map_err(CapiError::from)?;
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
        expression
            .write()
            .add_quadratic_term(u, v, bias)
            .map_err(CapiError::from)?;
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
        expression
            .write()
            .add_higher_order_term(variables, bias)
            .map_err(CapiError::from)?;
        Ok(())
    })
}

// pub expr_set_linear: unsafe extern "C" fn(*mut LmExpr, *const LmLinear) -> LmStatus,
/// Sets the linear term of an expression to the given linear term.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `linear` must be a valid linear handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_set_linear_term(
    expr: *mut LmExpr,
    linear: *const LmLinear,
) -> LmStatus {
    guard(|| {
        let mut expression = unsafe { expression_from_c(expr, "expr") }?.write();
        let linear = unsafe { ref_from_c(linear as *const Linear, "linear") }?;
        expression
            .set_linear(linear.clone())
            .map_err(CapiError::from)?;
        Ok(())
    })
}
// pub expr_set_quadratic: unsafe extern "C" fn(*mut LmExpr, *const LmQuadratic) -> LmStatus,
/// Sets the quadratic term of an expression to the given quadratic term.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `quadratic` must be a valid quadratic handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_set_quadratic_term(
    expr: *mut LmExpr,
    quadratic: *const LmQuadratic,
) -> LmStatus {
    guard(|| {
        let mut expression = unsafe { expression_from_c(expr, "expr") }?.write();

        let quadratic = if quadratic.is_null() {
            None
        } else {
            Some(unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?.clone())
        };

        expression
            .set_quadratic(quadratic)
            .map_err(CapiError::from)?;
        Ok(())
    })
}
// pub expr_set_higher_order: unsafe extern "C" fn(*mut LmExpr, *const LmHigherOrder) -> LmStatus,
/// Sets the higher-order term of an expression to the given higher-order term.
///
/// # Safety
/// `expr` must be a valid mutable expression handle created by this library.
/// `higher_order` must be a valid higher-order handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_set_higher_order_term(
    expr: *mut LmExpr,
    higher_order: *const LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let mut expression = unsafe { expression_from_c(expr, "expr") }?.write();

        let ho = if higher_order.is_null() {
            None
        } else {
            Some(unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?.clone())
        };
        expression.set_higher_order(ho).map_err(CapiError::from)?;
        Ok(())
    })
}
