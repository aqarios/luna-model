use luna_model::{
    core::{
        Expression,
        prelude::{DefaultEditable, HigherOrder, Linear, Quadratic},
    },
    types::Bias,
};

use crate::{
    LmEnv, LmExpr, LmHigherOrder, LmLinear, LmQuadratic, LmStatus,
    env::utils::environment_from_c,
    error::{CapiError, guard},
};

/// Frees an expression handle. Passing null is a no-op.
///
/// # Safety
/// `expr` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_free(expr: *mut LmExpr) {
    if !expr.is_null() {
        drop(unsafe { Box::from_raw(expr as *mut Expression) });
    }
}

/// Creates a new empty expression.
///
/// Returns a non-null handle that must be released with [`lm_expr_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_expr_default() -> *mut LmExpr {
    Box::into_raw(Box::new(Expression::default())) as *mut LmExpr
}

/// Creates a new empty expression with an environment.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_new(env: *const LmEnv, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let env = unsafe { environment_from_c(env) }?;
        let expr = Expression::empty(env.clone());
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;

        Ok(())
    })
}

/// Creates a new constant expression with an environment.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_new_constant(
    env: *const LmEnv,
    val: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let env = unsafe { environment_from_c(env) }?;
        let expr = Expression::constant(env.clone(), val);
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}

/// Creates a new expression from a constant double.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_double(cnst: f64, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let expr = Expression::with(|e| e.offset = cnst);
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}

/// Creates a new expression from a constant size_t.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_size_t(cnst: usize, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let expr = Expression::with(|e| e.offset = cnst as Bias);
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}

/// Creates a new expression from a linear term.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_linear(
    lin: *const LmLinear,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let lin = unsafe { (lin as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("lin"))?;
        let expr = Expression::with(|e| e.linear = lin.clone());
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}

/// Creates a new expression from a quadratic term.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_quadratic(
    quad: *const LmQuadratic,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let quad = unsafe { (quad as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quad"))?;
        let expr = Expression::with(|e| e.quadratic = Some(quad.clone()));
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}

/// Creates a new expression from a higher-order term.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_higher_order(
    ho: *const LmHigherOrder,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let ho = unsafe { (ho as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("ho"))?;
        let expr = Expression::with(|e| e.higher_order = Some(ho.clone()));
        *out = Box::into_raw(Box::new(expr)) as *mut LmExpr;
        Ok(())
    })
}
