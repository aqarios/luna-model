use crate::{
    LmBuffer, LmSol, LmStatus, LmValueSource,
    buffer::buffer_from_vec,
    error::{CapiError, guard},
    utils::{opt_from_c, out_from_c, shared_from_c, value_source_from_c},
};

/// Computes the conditional value at risk (CVaR) of the selected value series.
///
/// The implementation follows the discrete empirical definition used for a
/// finite set of sampled outcomes.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `doublbe` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_cvar(
    sol: *const LmSol,
    alpha: f64,
    toggle: *const LmValueSource,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let toggle = unsafe { opt_from_c(toggle) }.map(value_source_from_c);
        let out = unsafe { out_from_c(out) }?;
        *out = sol.cvar(alpha, toggle).map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Computes the count-weighted expectation value of the selected series.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `doublbe` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_expectation_value(
    sol: *const LmSol,
    toggle: *const LmValueSource,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let toggle = unsafe { opt_from_c(toggle) }.map(value_source_from_c);
        let out = unsafe { out_from_c(out) }?;
        *out = sol.expectation_value(toggle).map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Computes a Boltzmann-style temperature-weighted expectation value.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `doublbe` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_temperature_weighted(
    sol: *const LmSol,
    beta: f64,
    toggle: *const LmValueSource,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let toggle = unsafe { opt_from_c(toggle) }.map(value_source_from_c);
        let out = unsafe { out_from_c(out) }?;
        *out = sol
            .temperature_weighted(beta, toggle)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Returns the fraction of total sample mass marked as feasible.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `doublbe` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_feasibility_ratio(sol: *const LmSol, out: *mut f64) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.feasibility_ratio().map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Returns the name of the most frequently violated constraint, if available.
///
/// On success the name is written to `out` as a byte buffer that must be released
/// with [`lm_buffer_free`]. Returns `LmStatus::Absent` when no violated constraint
/// is available.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_highest_constraint_violations(
    sol: *const LmSol,
    out: *mut LmBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        match sol
            .highest_constraint_violations()
            .map_err(CapiError::errored)?
        {
            Some(name) => {
                *out = buffer_from_vec(name.into_bytes());
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "solution has no violated constraint".to_string(),
            )),
        }
    })
}
