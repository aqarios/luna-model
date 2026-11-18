use crate::{
    LmBoolMapBuffer, LmResultView, LmSampleView, LmStatus,
    buffer::bool_map_buffer_from_vec,
    error::CapiError,
    error::guard,
    res::ResultViewHandle,
    sample::SampleViewHandle,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Returns how often the result row occurred in the solution.
///
/// # Safety
/// `view` must be a valid result-view handle whose originating solution is still alive.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_counts(
    view: *const LmResultView,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let view = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = view.read(|view| view.counts());
        Ok(())
    })
}

/// Returns constraint feasibility for this result row.
///
/// # Safety
/// `view` must be valid and `out` must be writable buffer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_constraints(
    view: *const LmResultView,
    out: *mut LmBoolMapBuffer,
) -> LmStatus {
    guard(|| {
        let view = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        let values = view.read(|view| view.constraints()).ok_or_else(|| {
            CapiError::new(LmStatus::Absent, "result row has no constraint results")
        })?;
        *out = bool_map_buffer_from_vec(
            values
                .into_iter()
                .map(|(name, value)| (name, vec![value]))
                .collect(),
        );
        Ok(())
    })
}

/// Returns variable-bound feasibility for this result row.
///
/// # Safety
/// `view` must be valid and `out` must be writable buffer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_variable_bounds(
    view: *const LmResultView,
    out: *mut LmBoolMapBuffer,
) -> LmStatus {
    guard(|| {
        let view = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        let values = view.read(|view| view.variable_bounds()).ok_or_else(|| {
            CapiError::new(LmStatus::Absent, "result row has no variable-bound results")
        })?;
        *out = bool_map_buffer_from_vec(
            values
                .into_iter()
                .map(|(name, value)| (name, vec![value]))
                .collect(),
        );
        Ok(())
    })
}

/// Returns an owned sample-view snapshot for this result row.
///
/// # Safety
/// `view` must be valid and `out` must be writable handle storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_sample(
    view: *const LmResultView,
    out: *mut *mut LmSampleView,
) -> LmStatus {
    guard(|| {
        let handle = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(SampleViewHandle {
            sol: handle.sol.clone(),
            index: handle.index,
        });
        Ok(())
    })
}

/// Returns the model-evaluated objective value for the result row.
///
/// # Safety
/// `view` must be a valid result-view handle whose originating solution is still alive.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_obj_value(
    view: *const LmResultView,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let view = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        match view.read(|view| view.obj_value()) {
            Some(v) => {
                *out = v;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "result row has no objective value".to_string(),
            )),
        }
    })
}

/// Returns the raw solver-provided energy for the result row.
///
/// # Safety
/// `view` must be a valid result-view handle whose originating solution is still alive.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_raw_energy(
    view: *const LmResultView,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let view = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        match view.read(|view| view.raw_energy()) {
            Some(v) => {
                *out = v;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "result row has no raw energy".to_string(),
            )),
        }
    })
}

/// Returns whether the result row is globally feasible.
///
/// # Safety
/// `view` must be a valid result-view handle whose originating solution is still alive.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_feasible(
    view: *const LmResultView,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let view = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
        let out = unsafe { out_from_c(out) }?;
        match view.read(|view| view.feasible()) {
            Some(v) => {
                *out = v;
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "result row has no feasibility flag".to_string(),
            )),
        }
    })
}
