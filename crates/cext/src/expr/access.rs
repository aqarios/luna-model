use super::utils::expression_from_c;
use crate::{
    LmEnv, LmExpr, LmHigherOrder, LmLinear, LmQuadratic, LmStatus, LmVref, LmVrefBuffer, LmVtype,
    LmVtypeBuffer,
    buffer::{vref_buffer_from_vec, vtype_buffer_from_vec},
    error::guard,
    utils::vtype_to_c,
    utils::{handle_to_c, out_from_c, ref_from_c, storage_out_from_c},
};
use luna_model::core::prelude::VarRef;

/// Returns the environment of the expression.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmEnv *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_env(expr: *const LmExpr, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();

        unsafe { storage_out_from_c(out, expr.env.clone()) }
    })
}

/// Returns the constant offset of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_offset(expr: *const LmExpr, out: *mut f64) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();

        let out = unsafe { out_from_c(out) }?;
        *out = expr.offset;
        Ok(())
    })
}

/// Returns the linear terms of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `LmLinear *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_linear(expr: *const LmExpr, out: *mut *mut LmLinear) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();

        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(expr.linear.clone());
        Ok(())
    })
}

/// Returns a copy of the expression's quadratic terms.
///
/// On success, `*out` is set to an owned quadratic handle, or to NULL when
/// the expression has no quadratic terms.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `LmQuadratic *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_quadratic(
    expr: *const LmExpr,
    out: *mut *mut LmQuadratic,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();

        let out = unsafe { out_from_c(out) }?;
        *out = expr
            .quadratic
            .clone()
            .map(handle_to_c)
            .unwrap_or(std::ptr::null_mut());
        Ok(())
    })
}

/// Returns a copy of the expression's higher-order terms.
///
/// On success, `*out` is set to an owned higher-order handle, or to NULL when
/// the expression has no higher-order terms.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `LmHigherOrder *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_higher_order(
    expr: *const LmExpr,
    out: *mut *mut LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();

        let out = unsafe { out_from_c(out) }?;
        *out = expr
            .higher_order
            .clone()
            .map(handle_to_c)
            .unwrap_or(std::ptr::null_mut());
        Ok(())
    })
}

/// Returns the number of variables of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_num_vars(expr: *const LmExpr, out: *mut usize) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.num_vars();
        Ok(())
    })
}

/// Returns the distinct variable types used by the expression.
///
/// This matches `Expr::vtypes()` in the Rust API.
/// The returned array is owned by the caller and must be released with `lm_vtype_buffer_free`.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmVtypeBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_vtypes(expr: *const LmExpr, out: *mut LmVtypeBuffer) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        let vtypes = expr.vtypes().map(vtype_to_c).collect::<Vec<LmVtype>>();
        *out = vtype_buffer_from_vec(vtypes);
        Ok(())
    })
}

/// Returns the distinct variables used by the expression.
///
/// This matches `Expression::vars()` in the Rust API.
/// The returned array is owned by the caller and must be released with `lm_vref_buffer_free`.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmVrefBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_vars(expr: *const LmExpr, out: *mut LmVrefBuffer) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        let vars = expr.vars().map(handle_to_c).collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns the distinct variables used by the expression including inverted binary variables
/// explicitly.
///
/// This matches `Expression::vars()` in the Rust API.
/// The returned array is owned by the caller and must be released with `lm_vref_buffer_free`.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmVrefBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_complete_vars(
    expr: *const LmExpr,
    out: *mut LmVrefBuffer,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        let vars = expr
            .complete_vars()
            .map(handle_to_c)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns whether the expression contains `var`.
///
/// # Safety
/// `expr` and `var` must be valid handles and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_contains(
    expr: *const LmExpr,
    var: *const LmVref,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let var = unsafe { ref_from_c(var as *const VarRef, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = expr.contains(var);
        Ok(())
    })
}

/// Returns the degree of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_degree(expr: *const LmExpr, out: *mut usize) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.degree();
        Ok(())
    })
}

/// Returns whether the expression is constant.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_is_constant(expr: *const LmExpr, out: *mut bool) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.is_constant();
        Ok(())
    })
}

/// Returns whether the expression has at least one quadratic term.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_has_quadratic(expr: *const LmExpr, out: *mut bool) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.has_quadratic();
        Ok(())
    })
}

/// Returns whether the expression has at least one higher-order term.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_has_higher_order(expr: *const LmExpr, out: *mut bool) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let out = unsafe { out_from_c(out) }?;
        *out = expr.has_higher_order();
        Ok(())
    })
}
