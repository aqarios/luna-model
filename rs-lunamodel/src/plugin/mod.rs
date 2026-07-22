//! Authoring transpiler-pass plugins in **safe** Rust.
//!
//! The traits here are a 1:1 mirror of the native transpiler's pass traits — [`Artifact`],
//! [`Reversible`], [`TransformationPass`], [`PassContext`] — but over the `rs-lunamodel` facade
//! (`Model`/`Solution`), dispatched through the stable C ABI. A plugin author writes exactly what
//! they would for a native pass and never touches `unsafe`, `extern "C"`, raw handles, or the C
//! descriptor: [`descriptor`] and [`register_backward`] generate that glue by monomorphizing over
//! the concrete pass type, so all of the unsafe lives here, once.

use std::ffi::{CString, c_char, c_void};
use std::ptr;

use lunamodel_cext_vtable::transpiler::{
    LmBackwardFn, LmByteSink, LmPassContext, LmPassKind, LmPluginAnalysis, LmPluginTransform,
    LmStrSink,
};
use lunamodel_cext_vtable::{LmModel, LmSol, LmStatus};

use crate::ffi::{OwnedHandle, adopt};
use crate::{Error, Model, Solution, Status};

mod analysis;
mod artifact;
mod context;
mod pass;
mod reversible;

pub use analysis::{AnalysisPass, AnalysisResult};
pub use artifact::Artifact;
pub use context::PassContext;
pub use pass::TransformationPass;
pub use reversible::Reversible;

/// Builds the C-ABI descriptor for a plugin pass. luna-model takes ownership of the returned
/// descriptor's `user_data` and releases it when the pass is dropped. Authors rarely call this
/// directly — the Python macros and native drivers pass its result to the host.
pub fn descriptor<P: TransformationPass>(pass: P) -> crate::Result<LmPluginTransform> {
    let state = Box::new(State {
        id: CString::new(P::ID).map_err(|_| interior_nul("id"))?,
        name: CString::new(pass.name()).map_err(|_| interior_nul("name"))?,
        pass,
    });
    Ok(LmPluginTransform {
        user_data: Box::into_raw(state) as *mut c_void,
        id: trampoline_id::<P>,
        name: trampoline_name::<P>,
        requires: trampoline_requires::<P>,
        invalidates: trampoline_invalidates::<P>,
        forward: trampoline_forward::<P>,
        free: trampoline_free::<P>,
    })
}

/// Builds the C-ABI descriptor for a plugin **analysis** pass. luna-model takes ownership of the
/// returned descriptor's `user_data` and releases it when the pass is dropped. Authors rarely call
/// this directly — a driver passes its result to the host to run the analysis in a pipeline.
pub fn descriptor_analysis<A: AnalysisPass>(pass: A) -> crate::Result<LmPluginAnalysis> {
    let state = Box::new(AnalysisState {
        id: CString::new(A::ID).map_err(|_| interior_nul("id"))?,
        name: CString::new(pass.name()).map_err(|_| interior_nul("name"))?,
        provides: CString::new(A::PROVIDES).map_err(|_| interior_nul("provides"))?,
        result_type_tag: CString::new(<A::Output as AnalysisResult>::static_type_tag())
            .map_err(|_| interior_nul("result type tag"))?,
        pass,
    });
    Ok(LmPluginAnalysis {
        user_data: Box::into_raw(state) as *mut c_void,
        id: analysis_trampoline_id::<A>,
        name: analysis_trampoline_name::<A>,
        provides: analysis_trampoline_provides::<A>,
        result_type_tag: analysis_trampoline_result_tag::<A>,
        requires: analysis_trampoline_requires::<A>,
        run: analysis_trampoline_run::<A>,
        free: analysis_trampoline_free::<A>,
    })
}

/// Registers a plugin pass's reversal handler with the host, keyed by [`Reversible::ID`]. Call once
/// per pass type (typically from a plugin's registration hook) so the host can reverse later.
pub fn register_backward<P: TransformationPass>() -> crate::Result<()> {
    // Safety: `trampoline_backward::<P>` is a valid reversal callback; it needs no `user_data` and
    // dispatches to the associated `P::backward`.
    unsafe {
        register_plugin_backward(
            LmPassKind::Transformation,
            P::ID,
            trampoline_backward::<P>,
            ptr::null_mut(),
        )
    }
}

/// Low-level primitive behind [`register_backward`]: registers a raw reversal callback.
///
/// # Safety
/// `func` must be a valid reversal callback and `user_data` valid for as long as any record
/// produced by the pass may be reversed (typically the process lifetime).
pub unsafe fn register_plugin_backward(
    kind: LmPassKind,
    id: &str,
    func: LmBackwardFn,
    user_data: *mut c_void,
) -> crate::Result<()> {
    check!(register_plugin_backward(kind, !id, func, user_data))
}

/// Boxed plugin state behind the descriptor's `user_data`, caching the id/name C strings so their
/// trampolines can hand out stable borrowed pointers.
struct State<P: TransformationPass> {
    pass: P,
    id: CString,
    name: CString,
}

fn interior_nul(field: &str) -> Error {
    Error {
        status: Status::InvalidArgument,
        message: format!("plugin `{field}` contains an interior NUL byte"),
    }
}

unsafe fn emit_strings(items: &[String], sink: LmStrSink, ctx: *mut c_void) {
    for item in items {
        if let Ok(cstr) = CString::new(item.as_str()) {
            unsafe { sink(ctx, cstr.as_ptr()) };
        }
    }
}

unsafe extern "C" fn trampoline_id<P: TransformationPass>(user_data: *mut c_void) -> *const c_char {
    unsafe { (*(user_data as *const State<P>)).id.as_ptr() }
}

unsafe extern "C" fn trampoline_name<P: TransformationPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const State<P>)).name.as_ptr() }
}

unsafe extern "C" fn trampoline_requires<P: TransformationPass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const State<P>) };
    unsafe { emit_strings(state.pass.requires(), sink, ctx) };
}

unsafe extern "C" fn trampoline_invalidates<P: TransformationPass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const State<P>) };
    unsafe { emit_strings(state.pass.invalidates(), sink, ctx) };
}

unsafe extern "C" fn trampoline_forward<P: TransformationPass>(
    user_data: *mut c_void,
    model: *mut LmModel,
    ctx: *const LmPassContext,
    sink: LmByteSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    let state = unsafe { &*(user_data as *const State<P>) };
    // Borrow the lent handle, run the safe `forward`, then relinquish it without freeing.
    let mut model = match unsafe { adopt::<Model>(model) } {
        Ok(model) => model,
        Err(_) => return LmStatus::NullPointer,
    };
    let context = PassContext::from_raw(ctx);
    let result = state.pass.forward(&mut model, &context);
    let _ = model.into_raw_owned();
    match result {
        Ok(artifact) => match artifact.serialize() {
            Ok(bytes) => {
                unsafe { sink(sink_ctx, bytes.as_ptr(), bytes.len()) };
                LmStatus::Ok
            }
            Err(_) => LmStatus::Serialization,
        },
        Err(_) => LmStatus::Errored,
    }
}

unsafe extern "C" fn trampoline_backward<P: TransformationPass>(
    _user_data: *mut c_void,
    artifact: *const u8,
    len: usize,
    solution: *mut LmSol,
    out_solution: *mut *mut LmSol,
) -> LmStatus {
    let bytes = if len == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(artifact, len) }
    };
    // Take ownership of the incoming solution up front so every early return frees it correctly.
    let solution = match unsafe { adopt::<Solution>(solution) } {
        Ok(solution) => solution,
        Err(_) => return LmStatus::NullPointer,
    };
    let artifact = match <P::Artifact as Artifact>::deserialize(bytes) {
        Ok(artifact) => artifact,
        Err(_) => return LmStatus::Deserialization,
    };
    match P::backward(&artifact, solution) {
        Ok(result) => {
            unsafe { *out_solution = result.into_raw_owned() };
            LmStatus::Ok
        }
        Err(_) => LmStatus::Errored,
    }
}

unsafe extern "C" fn trampoline_free<P: TransformationPass>(user_data: *mut c_void) {
    drop(unsafe { Box::from_raw(user_data as *mut State<P>) });
}

/// Boxed plugin state behind an analysis descriptor's `user_data`, caching the id/name/provides/
/// result-tag C strings so their trampolines can hand out stable borrowed pointers.
struct AnalysisState<A: AnalysisPass> {
    pass: A,
    id: CString,
    name: CString,
    provides: CString,
    result_type_tag: CString,
}

unsafe extern "C" fn analysis_trampoline_id<A: AnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const AnalysisState<A>)).id.as_ptr() }
}

unsafe extern "C" fn analysis_trampoline_name<A: AnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const AnalysisState<A>)).name.as_ptr() }
}

unsafe extern "C" fn analysis_trampoline_provides<A: AnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const AnalysisState<A>)).provides.as_ptr() }
}

unsafe extern "C" fn analysis_trampoline_result_tag<A: AnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const AnalysisState<A>)).result_type_tag.as_ptr() }
}

unsafe extern "C" fn analysis_trampoline_requires<A: AnalysisPass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const AnalysisState<A>) };
    unsafe { emit_strings(state.pass.requires(), sink, ctx) };
}

unsafe extern "C" fn analysis_trampoline_run<A: AnalysisPass>(
    user_data: *mut c_void,
    model: *const LmModel,
    ctx: *const LmPassContext,
    sink: LmByteSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    let state = unsafe { &*(user_data as *const AnalysisState<A>) };
    // Borrow the lent read-only handle, run the safe `run`, then relinquish it without freeing.
    let model = match unsafe { adopt::<Model>(model as *mut LmModel) } {
        Ok(model) => model,
        Err(_) => return LmStatus::NullPointer,
    };
    let context = PassContext::from_raw(ctx);
    let result = state.pass.run(&model, &context);
    let _ = model.into_raw_owned();
    match result {
        Ok(output) => match output.serialize() {
            Ok(bytes) => {
                unsafe { sink(sink_ctx, bytes.as_ptr(), bytes.len()) };
                LmStatus::Ok
            }
            Err(_) => LmStatus::Serialization,
        },
        Err(_) => LmStatus::Errored,
    }
}

unsafe extern "C" fn analysis_trampoline_free<A: AnalysisPass>(user_data: *mut c_void) {
    drop(unsafe { Box::from_raw(user_data as *mut AnalysisState<A>) });
}
