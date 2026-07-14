use std::ffi::c_char;

use crate::{
    LmAssignment, LmSampleView, LmStatus, LmStrBuffer,
    buffer::str_buffer_from_vec,
    error::{CapiError, guard},
    sample::SampleViewHandle,
    utils::{assignment_to_c, out_from_c, ref_from_c, str_from_c},
};

fn handle<'a>(view: *const LmSampleView) -> Result<&'a SampleViewHandle, CapiError> {
    unsafe { ref_from_c(view as *const SampleViewHandle, "view") }
}

/// Returns the sample row index.
///
/// # Safety
/// `view` must be a valid sample-view handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sample_view_index(
    view: *const LmSampleView,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let view = handle(view)?;
        let out = unsafe { out_from_c(out) }?;
        *out = view.index;
        Ok(())
    })
}

/// Returns variable names in column order.
///
/// # Safety
/// `view` must be valid and `out` must be writable buffer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sample_view_variable_names(
    view: *const LmSampleView,
    out: *mut LmStrBuffer,
) -> LmStatus {
    guard(|| {
        let view = handle(view)?;
        let out = unsafe { out_from_c(out) }?;
        *out = str_buffer_from_vec(view.sol.variable_names());
        Ok(())
    })
}

/// Returns an assignment by variable name.
///
/// # Safety
/// `view` must be valid, `var` NUL-terminated, and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sample_view_assignment(
    view: *const LmSampleView,
    var: *const c_char,
    out: *mut LmAssignment,
) -> LmStatus {
    guard(|| {
        let view = handle(view)?;
        let var = unsafe { str_from_c(var, "var") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = assignment_to_c(
            view.sol
                .try_assignment(view.index, var)
                .map_err(CapiError::errored)?,
        );
        Ok(())
    })
}

/// Returns an assignment by column index.
///
/// # Safety
/// `view` must be valid and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sample_view_assignment_idx(
    view: *const LmSampleView,
    var: usize,
    out: *mut LmAssignment,
) -> LmStatus {
    guard(|| {
        let view = handle(view)?;
        let out = unsafe { out_from_c(out) }?;
        *out = assignment_to_c(
            view.sol
                .try_assignment_idx(view.index, var)
                .map_err(CapiError::errored)?,
        );
        Ok(())
    })
}
