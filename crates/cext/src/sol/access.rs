use std::ffi::c_char;

use crate::{
    LmAssignment, LmColumn, LmResultView, LmResultViewBuffer, LmSampleView, LmSampleViewBuffer,
    LmSol, LmStatus, LmStrBuffer,
    buffer::{result_view_buffer_from_vec, sample_view_buffer_from_vec, str_buffer_from_vec},
    error::{CapiError, guard},
    res::ResultViewHandle,
    sample::SampleViewHandle,
    utils::{
        assignment_to_c, handle_to_c, new_handle_out_from_c, out_from_c, shared_from_c, str_from_c,
    },
};

/// Returns the number of samples stored in the solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_n_samples(sol: *const LmSol, out: *mut usize) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.n_samples();
        Ok(())
    })
}

/// Returns the number of sample rows in the solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_len(sol: *const LmSol, out: *mut usize) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.len();
        Ok(())
    })
}

/// Returns the stored variable names in column order.
///
/// The returned buffer must be released with [`lm_str_buffer_free`].
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmStrBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_variable_names(
    sol: *const LmSol,
    out: *mut LmStrBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = str_buffer_from_vec(sol.variable_names());
        Ok(())
    })
}

/// Returns if the solution is empty.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_is_empty(sol: *const LmSol, out: *mut bool) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.is_empty();
        Ok(())
    })
}

/// Returns the number of variables represented in the solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_sample_len(sol: *const LmSol, out: *mut usize) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.sample_len();
        Ok(())
    })
}

/// Returns the value for a variable in a row of the solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `var` must point to a valid NUL-terminated C string.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_value(
    sol: *const LmSol,
    sample: usize,
    var: *const c_char,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let var = unsafe { str_from_c(var, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        match sol.value(sample, var) {
            Some(value) => {
                *out = *value;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                format!("solution has no value in sample '{sample}' for variable '{var}'"),
            )),
        }
    })
}

/// Returns the typed assignment for a variable in a row of the solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `var` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmAssignment` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_assignment(
    sol: *const LmSol,
    sample: usize,
    var: *const c_char,
    out: *mut LmAssignment,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let var = unsafe { str_from_c(var, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        let assignment = sol
            .try_assignment(sample, var)
            .map_err(CapiError::errored)?;
        *out = assignment_to_c(assignment);
        Ok(())
    })
}

/// Returns the typed assignment for a variable index in a row of the solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmAssignment` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_assignment_idx(
    sol: *const LmSol,
    sample: usize,
    var: usize,
    out: *mut LmAssignment,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let assignment = sol
            .try_assignment_idx(sample, var)
            .map_err(CapiError::errored)?;
        *out = assignment_to_c(assignment);
        Ok(())
    })
}

/// Returns a result-oriented view over the sample row at `index`.
///
/// The returned handle borrows `sol`: it must be released with
/// [`lm_result_view_free`] and must not outlive `sol`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmResultView *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_result(
    sol: *const LmSol,
    index: usize,
    out: *mut *mut LmResultView,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?;
        let exists = sol.read().result(index).is_some();
        let out = unsafe { out_from_c(out) }?;
        match exists {
            true => {
                *out = handle_to_c(ResultViewHandle {
                    sol: sol.clone(),
                    index,
                });
                Ok(())
            }
            false => Err(CapiError::new(
                LmStatus::Absent,
                format!("solution has no result row at index '{index}'"),
            )),
        }
    })
}

/// Returns result-oriented views over solution.
///
/// The returned handle borrows `sol`: it must be released with
/// [`lm_result_view_buffer_free`] and must not outlive `sol`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmResultViewBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_results(
    sol: *const LmSol,
    out: *mut LmResultViewBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?;
        let indices = sol
            .read()
            .results()
            .map(|result| result.idx)
            .collect::<Vec<_>>();
        let out = unsafe { out_from_c(out) }?;
        let views = indices
            .into_iter()
            .map(|index| {
                handle_to_c(ResultViewHandle {
                    sol: sol.clone(),
                    index,
                })
            })
            .collect::<Vec<*mut LmResultView>>();
        *out = result_view_buffer_from_vec(views);
        Ok(())
    })
}

/// Returns a sample-oriented view over the sample row at `index`.
///
/// The returned handle borrows `sol`: it must be released with
/// [`lm_sample_view_free`] and must not outlive `sol`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSampleView *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_sample_view(
    sol: *const LmSol,
    index: usize,
    out: *mut *mut LmSampleView,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?;
        let exists = sol.read().sample(index).is_some();
        let out = unsafe { out_from_c(out) }?;
        match exists {
            true => {
                *out = handle_to_c(SampleViewHandle {
                    sol: sol.clone(),
                    index,
                });
                Ok(())
            }
            false => Err(CapiError::new(
                LmStatus::Absent,
                format!("solution has no sample at index '{index}'"),
            )),
        }
    })
}

/// Returns sample-views over solution.
///
/// The returned handle borrows `sol`: it must be released with
/// [`lm_sample_view_buffer_free`] and must not outlive `sol`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSampleViewBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_sample_views(
    sol: *const LmSol,
    out: *mut LmSampleViewBuffer,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?;
        let indices = sol
            .read()
            .samples()
            .map(|sample| sample.idx)
            .collect::<Vec<_>>();
        let out = unsafe { out_from_c(out) }?;
        let views = indices
            .into_iter()
            .map(|index| {
                handle_to_c(SampleViewHandle {
                    sol: sol.clone(),
                    index,
                })
            })
            .collect::<Vec<*mut LmSampleView>>();
        *out = sample_view_buffer_from_vec(views);
        Ok(())
    })
}

/// Returns the best results in the solution.
///
/// The returned handle borrows `sol`: it must be released with
/// [`lm_result_view_buffer_free`] and must not outlive `sol`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmResultViewBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_best(sol: *const LmSol, out: *mut LmResultViewBuffer) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?;
        let out = unsafe { out_from_c(out) }?;
        let indices = {
            let solution = sol.read();
            solution
                .best()
                .ok_or_else(|| {
                    CapiError::new(
                        LmStatus::Absent,
                        "solution has no best result (missing feasibility or objective values)",
                    )
                })?
                .into_iter()
                .map(|result| result.idx)
                .collect::<Vec<_>>()
        };
        let views = indices
            .into_iter()
            .map(|index| {
                handle_to_c(ResultViewHandle {
                    sol: sol.clone(),
                    index,
                })
            })
            .collect::<Vec<*mut LmResultView>>();
        *out = result_view_buffer_from_vec(views);
        Ok(())
    })
}

/// Extracts a single sample row as a standalone one-row solution.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_extract(
    sol: *const LmSol,
    row: usize,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let ex = sol.extract(row);
        unsafe { new_handle_out_from_c(out, ex) }
    })
}

/// Returns a copy of the column stored for `var`.
///
/// The handle must be released with [`lm_column_free`].
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `var` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmColumn *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_column(
    sol: *const LmSol,
    var: *const c_char,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let var = unsafe { str_from_c(var, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        let col = sol.samples.get(var).ok_or_else(|| {
            CapiError::new(
                LmStatus::Absent,
                format!("solution has no column for variable '{var}'"),
            )
        })?;
        *out = handle_to_c(col.clone());
        Ok(())
    })
}
