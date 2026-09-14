//! Bulk per-sample solution lookup — dispatches straight into
//! [`lunamodel_core::ops::SolutionLookup`], no reimplementation here.
//!
//! Exists to avoid one ABI round-trip per value in hot per-sample loops (e.g.
//! samplers): build once from an env + solution, `update` in place per sample
//! row, read the flat lookup buffer, and `write_sol` the result back.

use lunamodel_core::ops::SolutionLookup;
use lunamodel_core::solution::sample::SampleView as CoreSampleView;

use crate::{
    LmEnv, LmSampleView, LmSol, LmSolutionLookup, LmStatus,
    error::{CapiError, guard},
    sample::SampleViewHandle,
    utils::{
        drop_from_c, handle_to_c, mut_from_c, out_from_c, ref_from_c, shared_from_c, slice_from_c,
        storage_from_c,
    },
};

/// Frees a solution-lookup handle. Passing null is a no-op.
///
/// # Safety
/// `lookup` must originate from [`lm_sol_lookup_new`] and must not be used or
/// freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_lookup_free(lookup: *mut LmSolutionLookup) {
    unsafe { drop_from_c(lookup as *mut SolutionLookup) }
}

/// Builds a bulk per-sample lookup table over `env`'s variables for `sol`.
///
/// # Safety
/// `env` and `sol` must be valid handles and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_lookup_new(
    env: *const LmEnv,
    sol: *const LmSol,
    out: *mut *mut LmSolutionLookup,
) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;
        let env = env.read_arc();
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let lookup = SolutionLookup::new(&env, &sol).map_err(CapiError::from)?;
        *unsafe { out_from_c(out) }? = handle_to_c(lookup);
        Ok(())
    })
}

/// Loads one sample row's values into the lookup table, in place.
///
/// # Safety
/// `lookup` and `view` must be valid handles.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_lookup_update(
    lookup: *mut LmSolutionLookup,
    view: *const LmSampleView,
) -> LmStatus {
    guard(|| {
        let lookup = unsafe { mut_from_c(lookup as *mut SolutionLookup, "lookup") }?;
        let view = unsafe { ref_from_c(view as *const SampleViewHandle, "view") }?;
        let sol = view.sol.read();
        let sample = CoreSampleView {
            sol: &sol,
            idx: view.index,
        };
        lookup.update(&sample).map_err(CapiError::from)?;
        Ok(())
    })
}

/// Returns a borrowed view of the current lookup values, indexed by variable id.
///
/// The pointer remains valid until the lookup table is next updated or freed.
///
/// # Safety
/// `lookup`, `data`, and `len` must be valid pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_lookup_values(
    lookup: *const LmSolutionLookup,
    data: *mut *const f64,
    len: *mut usize,
) -> LmStatus {
    guard(|| {
        let lookup = unsafe { ref_from_c(lookup as *const SolutionLookup, "lookup") }?;
        *unsafe { out_from_c(data) }? = lookup.lu.as_ptr();
        *unsafe { out_from_c(len) }? = lookup.lu.len();
        Ok(())
    })
}

/// Overwrites row `idx` of `sol` with `values` (indexed by variable id), using
/// `lookup`'s variable mapping.
///
/// `values` is typically a caller-owned copy of a prior [`lm_sol_lookup_values`]
/// call, mutated locally (e.g. by a sampler's hot per-flip loop) without any
/// further ABI round-trips, then written back once per sample here.
///
/// # Safety
/// `lookup` and `sol` must be valid handles. `values` must be null when `len`
/// is zero or point to `len` readable values matching `lookup`'s variable count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_lookup_write_sol(
    lookup: *mut LmSolutionLookup,
    idx: u32,
    values: *const f64,
    len: usize,
    sol: *mut LmSol,
) -> LmStatus {
    guard(|| {
        let lookup = unsafe { mut_from_c(lookup as *mut SolutionLookup, "lookup") }?;
        let values = unsafe { slice_from_c(values, len, "values") }?;
        if values.len() != lookup.lu.len() {
            return Err(CapiError::invalid_argument(format!(
                "expected {} values, got {}",
                lookup.lu.len(),
                values.len()
            )));
        }
        lookup.lu.copy_from_slice(values);

        let sol = unsafe { shared_from_c(sol, "sol") }?;
        let mut sol = sol.write_arc();
        lookup.write_sol(idx, &mut sol);
        Ok(())
    })
}
