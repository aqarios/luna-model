//! Property accessors over a solution's stored per-sample metadata.

use crate::{
    LmBoolMapBuffer, LmIndexMapItemBuffer, LmSense, LmSol, LmStatus, LmTiming,
    buffer::{bool_map_buffer_from_vec, index_map_item_buffer_from_vec},
    error::{CapiError, guard},
    utils::{handle_to_c, out_from_c, sense_to_c, shared_from_c},
};

/// Returns the `(variable name, column)` entries of the solution's sample map.
///
/// The returned buffer must be released with [`lm_index_map_buffer_free`].
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmIndexMapItemBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_samples(
    sol: *const LmSol,
    out: *mut LmIndexMapItemBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let items = sol
            .samples
            .iter()
            .map(|(name, col)| (name.clone(), col.clone()))
            .collect();
        *out = index_map_item_buffer_from_vec(items);
        Ok(())
    })
}

/// Returns the per-sample occurrence counts as a slice borrowed from `sol`.
///
/// On success `out_data`/`out_len` describe a slice into `sol`: the pointer is
/// valid only while `sol` is alive and unmodified, and must not be freed.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out_data` must point to writable `const uintptr_t *` storage.
/// `out_len` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_counts(
    sol: *const LmSol,
    out_data: *mut *const usize,
    out_len: *mut usize,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out_data = unsafe { out_from_c(out_data) }?;
        let out_len = unsafe { out_from_c(out_len) }?;
        *out_data = sol.counts.as_ptr();
        *out_len = sol.counts.len();
        Ok(())
    })
}

/// Returns the raw solver-provided energies as a slice borrowed from `sol`.
///
/// On success `out_data`/`out_len` describe a slice into `sol`: the pointer is
/// valid only while `sol` is alive and unmodified, and must not be freed.
/// Returns `LmStatus::Absent` when raw energies were not evaluated.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out_data` must point to writable `const double *` storage.
/// `out_len` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_raw_energies(
    sol: *const LmSol,
    out_data: *mut *const f64,
    out_len: *mut usize,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out_data = unsafe { out_from_c(out_data) }?;
        let out_len = unsafe { out_from_c(out_len) }?;
        let series = sol.raw_energies.as_ref().ok_or_else(|| {
            CapiError::new(LmStatus::Absent, "solution has no raw energies".to_string())
        })?;
        *out_data = series.as_ptr();
        *out_len = series.len();
        Ok(())
    })
}

/// Returns the model-evaluated objective values as a slice borrowed from `sol`.
///
/// On success `out_data`/`out_len` describe a slice into `sol`: the pointer is
/// valid only while `sol` is alive and unmodified, and must not be freed.
/// Returns `LmStatus::Absent` when objective values were not evaluated.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out_data` must point to writable `const double *` storage.
/// `out_len` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_obj_values(
    sol: *const LmSol,
    out_data: *mut *const f64,
    out_len: *mut usize,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out_data = unsafe { out_from_c(out_data) }?;
        let out_len = unsafe { out_from_c(out_len) }?;
        let series = sol.obj_values.as_ref().ok_or_else(|| {
            CapiError::new(
                LmStatus::Absent,
                "solution has no objective values".to_string(),
            )
        })?;
        *out_data = series.as_ptr();
        *out_len = series.len();
        Ok(())
    })
}

/// Returns the per-sample feasibility flags as a slice borrowed from `sol`.
///
/// On success `out_data`/`out_len` describe a slice into `sol`: the pointer is
/// valid only while `sol` is alive and unmodified, and must not be freed.
/// Returns `LmStatus::Absent` when feasibility was not evaluated.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out_data` must point to writable `const bool *` storage.
/// `out_len` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_feasible(
    sol: *const LmSol,
    out_data: *mut *const bool,
    out_len: *mut usize,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out_data = unsafe { out_from_c(out_data) }?;
        let out_len = unsafe { out_from_c(out_len) }?;
        let series = sol.feasible.as_ref().ok_or_else(|| {
            CapiError::new(
                LmStatus::Absent,
                "solution has no feasibility flags".to_string(),
            )
        })?;
        *out_data = series.as_ptr();
        *out_len = series.len();
        Ok(())
    })
}

/// Returns the per-constraint feasibility flags keyed by constraint name.
///
/// The returned buffer must be released with [`lm_bool_map_buffer_free`].
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmBoolMapBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_constraints(
    sol: *const LmSol,
    out: *mut LmBoolMapBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let entries = sol
            .constraints
            .iter()
            .map(|(name, flags)| (name.clone(), flags.clone()))
            .collect();
        *out = bool_map_buffer_from_vec(entries);
        Ok(())
    })
}

/// Returns the per-variable bound-check flags keyed by variable name.
///
/// The returned buffer must be released with [`lm_bool_map_buffer_free`].
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmBoolMapBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_variable_bounds(
    sol: *const LmSol,
    out: *mut LmBoolMapBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let entries = sol
            .variable_bounds
            .iter()
            .map(|(name, flags)| (name.clone(), flags.clone()))
            .collect();
        *out = bool_map_buffer_from_vec(entries);
        Ok(())
    })
}

/// Returns a copy of the solution's timing metadata as an owned handle.
///
/// The handle must be released with [`lm_timing_free`]. Returns
/// `LmStatus::Absent` when the solution has no timing metadata.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmTiming *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_timing(sol: *const LmSol, out: *mut *mut LmTiming) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let timing = sol.timing.ok_or_else(|| {
            CapiError::new(LmStatus::Absent, "solution has no timing".to_string())
        })?;
        *out = handle_to_c(timing);
        Ok(())
    })
}

/// Returns the optimization sense the solution was created with.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSense` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_sense(sol: *const LmSol, out: *mut LmSense) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sense_to_c(sol.sense);
        Ok(())
    })
}
