//! `#[no_mangle]` C entry point backing a plugin pass's read access to analysis results.
//!
//! `LmPassContext` is represented as an [`AnalysisManager`]: the plugin adapter lends the manager
//! the transpiler holds during a run (borrowed, see `lunamodel-cext-plugin`), reaching the reader
//! below as `*const AnalysisManager`. The context is read-only, exactly like the native
//! `PassContext`; the manager is populated by the host as analyses run, never by a plugin.

use std::ffi::{c_char, c_void};

use lunamodel_transpiler::AnalysisManager;
use lunamodel_capi::LmStatus;
use lunamodel_capi::transpiler::{LmByteSink, LmPassContext};

use crate::error::{CapiError, guard};
use crate::utils::str_from_c;

/// Reads a plugin analysis result deposited under `name` earlier in the same run, verifies its
/// recorded type tag matches `expected_type_tag`, and hands the serialized bytes to `sink`.
///
/// Returns `Absent` when nothing is stored under `name`, and `InvalidArgument` when a result is
/// present but was produced by a different type than `expected_type_tag`; `sink` is invoked exactly
/// once (with the result bytes) only on `Ok`.
///
/// # Safety
/// `ctx` must be a live `*const LmPassContext` lent to a plugin during `forward`/`run` (not
/// retained past the call). `name` and `expected_type_tag` must be valid NUL-terminated UTF-8 C
/// strings. `sink` must be a valid callback that copies the bytes before returning.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_pass_context_get_analysis(
    ctx: *const LmPassContext,
    name: *const c_char,
    expected_type_tag: *const c_char,
    sink: LmByteSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    guard(|| {
        if ctx.is_null() {
            return Err(CapiError::null_pointer("ctx"));
        }
        let manager = unsafe { &*ctx.cast::<AnalysisManager>() };
        let name = unsafe { str_from_c(name, "name") }?;
        let expected = unsafe { str_from_c(expected_type_tag, "expected_type_tag") }?;

        match manager.get_serialized(name) {
            None => Err(CapiError::new(
                LmStatus::Absent,
                format!("no analysis result stored under '{name}'"),
            )),
            Some(result) if result.type_tag != expected => Err(CapiError::new(
                LmStatus::InvalidArgument,
                format!(
                    "analysis '{name}' has type tag '{}', but '{expected}' was expected",
                    result.type_tag
                ),
            )),
            Some(result) => {
                unsafe { sink(sink_ctx, result.bytes.as_ptr(), result.bytes.len()) };
                Ok(())
            }
        }
    })
}
