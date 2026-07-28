//! `#[no_mangle]` C entry points backing a meta-analysis plugin's read-only view of the pipeline
//! steps that follow it (`LmStepList`).
//!
//! The view is a borrowed [`StepListView`] (defined in `lunamodel-capi-plugin`, created by the
//! meta-analysis adapter for the duration of one `run` call). These entries cast the lent
//! `*const LmStepList` back to it and answer per-step queries; list-valued answers (name, requires,
//! provides, invalidates) are delivered through an `LmStrSink` so no ownership crosses the ABI.

use std::ffi::{CString, c_char, c_void};

use lunamodel_capi::LmStatus;
use lunamodel_capi::plugin::{LmPassKind, LmStepList, LmStrSink};
use lunamodel_capi_plugin::StepListView;

use crate::error::{CapiError, guard};

/// Casts a lent `*const LmStepList` back to the borrowed view it wraps.
///
/// # Safety
/// `steps` must be a live `*const LmStepList` lent to a plugin during `run` (not retained past it).
unsafe fn view<'a>(steps: *const LmStepList) -> Result<&'a StepListView, CapiError> {
    if steps.is_null() {
        return Err(CapiError::null_pointer("steps"));
    }
    Ok(unsafe { &*steps.cast::<StepListView>() })
}

/// Error for an out-of-range step index.
fn out_of_bounds(index: usize) -> CapiError {
    CapiError::new(
        LmStatus::InvalidArgument,
        format!("step index {index} is out of bounds"),
    )
}

/// Emits each string through `sink` (skipping any with an interior NUL, which cannot be a C string).
unsafe fn emit(items: &[String], sink: LmStrSink, sink_ctx: *mut c_void) {
    for item in items {
        if let Ok(c) = CString::new(item.as_str()) {
            unsafe { sink(sink_ctx, c.as_ptr() as *const c_char) };
        }
    }
}

/// Writes the number of steps in the lent view to `out_len`.
///
/// # Safety
/// `steps` must be a live `*const LmStepList`; `out_len` must be a valid writable `usize`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_step_list_len(
    steps: *const LmStepList,
    out_len: *mut usize,
) -> LmStatus {
    guard(|| {
        let view = unsafe { view(steps) }?;
        if out_len.is_null() {
            return Err(CapiError::null_pointer("out_len"));
        }
        unsafe { *out_len = view.len() };
        Ok(())
    })
}

/// Writes the kind of the step at `index` to `out_kind`. `InvalidArgument` if out of bounds.
///
/// # Safety
/// `steps` must be a live `*const LmStepList`; `out_kind` must be a valid writable `LmPassKind`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_step_list_kind(
    steps: *const LmStepList,
    index: usize,
    out_kind: *mut LmPassKind,
) -> LmStatus {
    guard(|| {
        let view = unsafe { view(steps) }?;
        if out_kind.is_null() {
            return Err(CapiError::null_pointer("out_kind"));
        }
        let kind = view.kind(index).ok_or_else(|| out_of_bounds(index))?;
        unsafe { *out_kind = kind };
        Ok(())
    })
}

/// Emits the name of the step at `index` via `sink` (once). `InvalidArgument` if out of bounds.
///
/// # Safety
/// `steps` must be a live `*const LmStepList`; `sink` must copy the string before returning.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_step_list_name(
    steps: *const LmStepList,
    index: usize,
    sink: LmStrSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    guard(|| {
        let view = unsafe { view(steps) }?;
        let name = view.name(index).ok_or_else(|| out_of_bounds(index))?;
        unsafe { emit(std::slice::from_ref(&name), sink, sink_ctx) };
        Ok(())
    })
}

/// Emits the analysis names the step at `index` requires, one per `sink` call.
///
/// # Safety
/// `steps` must be a live `*const LmStepList`; `sink` must copy each string before returning.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_step_list_requires(
    steps: *const LmStepList,
    index: usize,
    sink: LmStrSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    guard(|| {
        let view = unsafe { view(steps) }?;
        let items = view.requires(index).ok_or_else(|| out_of_bounds(index))?;
        unsafe { emit(&items, sink, sink_ctx) };
        Ok(())
    })
}

/// Emits the analysis names the step at `index` provides, one per `sink` call.
///
/// # Safety
/// `steps` must be a live `*const LmStepList`; `sink` must copy each string before returning.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_step_list_provides(
    steps: *const LmStepList,
    index: usize,
    sink: LmStrSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    guard(|| {
        let view = unsafe { view(steps) }?;
        let items = view.provides(index).ok_or_else(|| out_of_bounds(index))?;
        unsafe { emit(&items, sink, sink_ctx) };
        Ok(())
    })
}

/// Emits the analysis names the step at `index` invalidates, one per `sink` call.
///
/// # Safety
/// `steps` must be a live `*const LmStepList`; `sink` must copy each string before returning.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_step_list_invalidates(
    steps: *const LmStepList,
    index: usize,
    sink: LmStrSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    guard(|| {
        let view = unsafe { view(steps) }?;
        let items = view.invalidates(index).ok_or_else(|| out_of_bounds(index))?;
        unsafe { emit(&items, sink, sink_ctx) };
        Ok(())
    })
}
