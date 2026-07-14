//! Expression evaluation entry points for the C ABI.
//!
//! Thin wrappers over `core`'s `Expression::evaluate_sample` /
//! `evaluate_sampleset`; no evaluation logic lives here.

use std::collections::HashMap;
use std::ffi::c_char;

use luna_model::core::{Expression, Solution, TryIndex};
use luna_model::error::LunaModelError;

use crate::buffer::f64_buffer_from_vec;
use crate::error::{CapiError, guard};
use crate::utils::{out_from_c, ref_from_c, slice_from_c, str_from_c};
use crate::{LmExpr, LmF64Buffer, LmSol, LmStatus};

/// A name-keyed sample built from parallel C arrays; adapts to `TryIndex` so it
/// can be handed to `core`'s evaluation without duplicating any logic.
struct NameSample(HashMap<String, f64>);

impl TryIndex<&str> for NameSample {
    type Output = f64;
    type Err = LunaModelError;

    fn try_index(&self, var: &str) -> Result<&f64, LunaModelError> {
        self.0
            .get(var)
            .ok_or_else(|| LunaModelError::VariableNotExisting(var.into()))
    }
}

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
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let names = unsafe { slice_from_c(names, len, "names") }?;
        let values = unsafe { slice_from_c(values, len, "values") }?;
        let mut map = HashMap::with_capacity(len);
        for (&name, &value) in names.iter().zip(values) {
            map.insert(unsafe { str_from_c(name, "name") }?.to_owned(), value);
        }
        let out = unsafe { out_from_c(out) }?;
        *out = expr
            .evaluate_sample(&NameSample(map))
            .map_err(CapiError::errored)?;
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
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        let sol = unsafe { ref_from_c(sol as *const Solution, "sol") }?;
        let values = expr
            .evaluate_sampleset(sol.samples())
            .map_err(CapiError::errored)?;
        let out = unsafe { out_from_c(out) }?;
        *out = f64_buffer_from_vec(values);
        Ok(())
    })
}
