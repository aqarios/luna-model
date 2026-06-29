use luna_model::core::Expression;

use crate::{
    LmEnv, LmExpr, LmHigherOrderTerm, LmLinearTerm, LmQuadraticTerm, LmStatus, LmVref,
    LmVrefBuffer, LmVtype, LmVtypeBuffer,
    buffer::{vref_buffer_from_vec, vtype_buffer_from_vec},
    error::{CapiError, guard},
    utils::vtype_to_c,
};

/// Returns the environment of the expression.
///
/// # Safety
/// `expr` must be a valid expression handle created by this library.
/// `out` must point to writable `LmEnv` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_env(expr: *const LmExpr, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(expr.env.clone())) as *mut LmEnv;
        Ok(())
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
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.offset;
        Ok(())
    })
}

/// Returns the linear terms of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `LmLinearTerm` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_linear(
    expr: *const LmExpr,
    out: *mut *mut LmLinearTerm,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(expr.linear.clone())) as *mut LmLinearTerm;
        Ok(())
    })
}

/// Returns the quadratic terms of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `LmQuadraticTerm` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_quadratic(
    expr: *const LmExpr,
    out: *mut *mut LmQuadraticTerm,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(expr.quadratic.clone())) as *mut LmQuadraticTerm;
        Ok(())
    })
}

/// Returns the higher-order terms of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `LmHigherOrderTerm` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_higher_order(
    expr: *const LmExpr,
    out: *mut *mut LmHigherOrderTerm,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(expr.higher_order.clone())) as *mut LmHigherOrderTerm;
        Ok(())
    })
}

/// Returns the number of variables of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_num_vars(expr: *const LmExpr, out: *mut usize) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vars = expr
            .vars()
            .map(|v| Box::into_raw(Box::new(v)) as *mut LmVref)
            .collect::<Vec<*mut LmVref>>();
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
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let vars = expr
            .complete_vars()
            .map(|v| Box::into_raw(Box::new(v)) as *mut LmVref)
            .collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Returns the degree of the expression.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_degree(expr: *const LmExpr, out: *mut usize) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.degree();
        Ok(())
    })
}

/// Returns if the expression is constant.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_is_constant(expr: *const LmExpr, out: *mut bool) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.is_constant();
        Ok(())
    })
}

/// Returns if the expression has at least one quadratic term.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_has_quadratic(expr: *const LmExpr, out: *mut bool) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.has_quadratic();
        Ok(())
    })
}

/// Returns if the expression has at least one higher-order term.
///
/// # Safety
/// `expr` must be a valid expr handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_has_higher_order(expr: *const LmExpr, out: *mut bool) -> LmStatus {
    guard(|| {
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = expr.has_higher_order();
        Ok(())
    })
}
