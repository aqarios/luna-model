//! Expression evaluation entry points for the C ABI.
//!
//! Thin wrappers over `core`'s `Expression::evaluate_sample` /
//! `evaluate_sampleset`; no evaluation logic lives here.

use std::ffi::c_char;

use super::utils::expression_from_c;
use crate::buffer::f64_buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::eval::NameSample;
use crate::utils::{out_from_c, shared_from_c};
use crate::{LmExpr, LmF64Buffer, LmSol, LmStatus};

/// Evaluates the expression against a single name-keyed sample.
///
/// `names` and `values` are parallel arrays of length `len`.
///
/// # Safety
/// `expr` must be a valid expression handle. `names`/`values` must point to
/// `len` valid elements; `out` must point to writable `f64` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_evaluate_sample(
    expr: *const LmExpr,
    names: *const *const c_char,
    values: *const f64,
    len: usize,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let sample = unsafe { NameSample::from_c(names, values, len) }?;
        let out = unsafe { out_from_c(out) }?;
        *out = expr.evaluate_sample(&sample).map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Evaluates the expression across every sample row of `sol`.
///
/// # Safety
/// `expr` and `sol` must be valid handles; `out` must point to writable
/// `LmF64Buffer` storage. The returned buffer must be freed with
/// `lm_f64_buffer_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_expr_evaluate(
    expr: *const LmExpr,
    sol: *const LmSol,
    out: *mut LmF64Buffer,
) -> LmStatus {
    guard(|| {
        let expr = unsafe { expression_from_c(expr, "expr") }?.read();
        let sol = unsafe { shared_from_c(sol, "sol") }?.read();
        let values = expr
            .evaluate_sampleset(sol.samples())
            .map_err(CapiError::errored)?;
        let out = unsafe { out_from_c(out) }?;
        *out = f64_buffer_from_vec(values);
        Ok(())
    })
}
