use crate::utils::{drop_from_c, handle_to_c, out_from_c, ref_from_c};
use luna_model::{
    core::{
        Expression,
        prelude::{DefaultEditable, HigherOrder, Linear, Quadratic},
    },
    types::Bias,
};

use crate::{
    LmEnv, LmExpr, LmHigherOrder, LmLinear, LmQuadratic, LmStatus, env::utils::environment_from_c,
    error::guard,
};

/// Frees an expression handle. Passing null is a no-op.
///
/// # Safety
/// `expr` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_free(expr: *mut LmExpr) {
    unsafe { drop_from_c(expr as *mut Expression) }
}

/// Creates a new empty expression.
///
/// Returns a non-null handle that must be released with [`lm_expr_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_expr_default() -> *mut LmExpr {
    handle_to_c(Expression::default())
}

/// Creates a new empty expression with an environment.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_new(env: *const LmEnv, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;

        let env = unsafe { environment_from_c(env) }?;
        let expr = Expression::empty(env.clone());
        *out = handle_to_c(expr);

        Ok(())
    })
}

/// Creates a new constant expression with an environment.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_new_constant(
    env: *const LmEnv,
    val: f64,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let env = unsafe { environment_from_c(env) }?;
        let expr = Expression::constant(env.clone(), val);
        *out = handle_to_c(expr);
        Ok(())
    })
}

/// Creates a new expression from a constant double.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_double(cnst: f64, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let expr = Expression::with(|e| e.offset = cnst);
        *out = handle_to_c(expr);
        Ok(())
    })
}

/// Creates a new expression from a constant size_t.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_size_t(cnst: usize, out: *mut *mut LmExpr) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let expr = Expression::with(|e| e.offset = cnst as Bias);
        *out = handle_to_c(expr);
        Ok(())
    })
}

/// Creates a new expression from a linear term.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `lin` must be a valid linear handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_linear(
    lin: *const LmLinear,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let lin = unsafe { ref_from_c(lin as *const Linear, "lin") }?;
        let expr = Expression::with(|e| e.linear = lin.clone());
        *out = handle_to_c(expr);
        Ok(())
    })
}

/// Creates a new expression from a quadratic term.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `quad` must be a valid quadratic handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_quadratic(
    quad: *const LmQuadratic,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let quad = unsafe { ref_from_c(quad as *const Quadratic, "quad") }?;
        let expr = Expression::with(|e| e.quadratic = Some(quad.clone()));
        *out = handle_to_c(expr);
        Ok(())
    })
}

/// Creates a new expression from a higher-order term.
///
/// `out` receives an owned expression handle that must be freed with `lm_expr_free`.
///
/// # Safety
/// `ho` must be a valid higher-order handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_from_higher_order(
    ho: *const LmHigherOrder,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        let ho = unsafe { ref_from_c(ho as *const HigherOrder, "ho") }?;
        let expr = Expression::with(|e| e.higher_order = Some(ho.clone()));
        *out = handle_to_c(expr);
        Ok(())
    })
}
