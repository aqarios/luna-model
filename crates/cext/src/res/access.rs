use crate::{
    LmResultView, LmStatus,
    error::CapiError,
    error::guard,
    utils::{out_from_c, result_view_from_c},
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
        let view = unsafe { result_view_from_c(view) }?;
        let out = unsafe { out_from_c(out) }?;
        *out = view.counts();
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
        let view = unsafe { result_view_from_c(view) }?;
        let out = unsafe { out_from_c(out) }?;
        match view.obj_value() {
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
        let view = unsafe { result_view_from_c(view) }?;
        let out = unsafe { out_from_c(out) }?;
        match view.raw_energy() {
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
        let view = unsafe { result_view_from_c(view) }?;
        let out = unsafe { out_from_c(out) }?;
        match view.feasible() {
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
