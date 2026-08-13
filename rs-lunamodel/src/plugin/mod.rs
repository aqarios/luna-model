//! Authoring transpiler-pass plugins in **safe** Rust.
//!
//! The traits here are a 1:1 mirror of the native transpiler's pass traits — [`Artifact`],
//! [`Reversible`], [`TransformationPass`], [`AnalysisPass`], [`CompositePass`], [`PassContext`] —
//! but over the `rs-lunamodel` facade (`Model`/`Solution`), dispatched through the stable C ABI. A
//! plugin author writes exactly what they would for a native pass and never touches `unsafe`,
//! `extern "C"`, raw handles, or the C descriptor: [`descriptor`] and [`register_backward`] (and
//! their per-kind siblings) generate that glue by monomorphizing over the concrete pass type, so
//! all of the unsafe lives here, once.

use std::ffi::{CString, c_char, c_void};
use std::ptr;

use lunamodel_capi::plugin::{
    LmBackwardFn, LmByteSink, LmControlFlowPlan, LmPassContext, LmPassKind, LmPluginAnalysis,
    LmPluginComposite, LmPluginControlFlow, LmPluginMetaAnalysis, LmPluginTransform, LmStepList,
    LmStrSink,
};
use lunamodel_capi::{LmModel, LmSol, LmStatus};

use prost::Message;

use crate::ffi::{OwnedHandle, adopt};
use crate::{Error, Model, Solution, Status};

mod analysis;
mod artifact;
mod composite;
mod context;
mod control_flow;
mod meta;
mod pass;
mod reversible;
mod steps;

pub use analysis::{AnalysisPass, AnalysisResult};
pub use artifact::Artifact;
pub use composite::CompositePass;
pub use context::PassContext;
pub use control_flow::{ControlFlowPass, ControlFlowPlan};
pub use meta::MetaAnalysisPass;
pub use pass::TransformationPass;
pub use reversible::Reversible;
pub use steps::{Step, StepKind, StepList};

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
        requires_analyses: trampoline_requires::<P>,
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
        requires_analyses: analysis_trampoline_requires::<A>,
        run: analysis_trampoline_run::<A>,
        free: analysis_trampoline_free::<A>,
    })
}

/// Builds the C-ABI descriptor for a plugin **composite** pass — a fused transform + analysis.
/// luna-model takes ownership of the returned descriptor's `user_data` and releases it when the
/// pass is dropped. Authors rarely call this directly — a driver passes its result to the host.
pub fn descriptor_composite<C: CompositePass>(pass: C) -> crate::Result<LmPluginComposite> {
    let state = Box::new(CompositeState {
        id: CString::new(C::ID).map_err(|_| interior_nul("id"))?,
        name: CString::new(pass.name()).map_err(|_| interior_nul("name"))?,
        provides: CString::new(C::PROVIDES).map_err(|_| interior_nul("provides"))?,
        result_type_tag: CString::new(<C::Output as AnalysisResult>::static_type_tag())
            .map_err(|_| interior_nul("result type tag"))?,
        pass,
    });
    Ok(LmPluginComposite {
        user_data: Box::into_raw(state) as *mut c_void,
        id: composite_trampoline_id::<C>,
        name: composite_trampoline_name::<C>,
        provides: composite_trampoline_provides::<C>,
        result_type_tag: composite_trampoline_result_tag::<C>,
        requires_analyses: composite_trampoline_requires::<C>,
        invalidates: composite_trampoline_invalidates::<C>,
        forward: composite_trampoline_forward::<C>,
        free: composite_trampoline_free::<C>,
    })
}

/// Builds the C-ABI descriptor for a plugin **meta-analysis** pass. luna-model takes ownership of
/// the returned descriptor's `user_data` and releases it when the pass is dropped. Authors rarely
/// call this directly — a driver passes its result to the host to run the pass in a pipeline.
pub fn descriptor_meta_analysis<M: MetaAnalysisPass>(
    pass: M,
) -> crate::Result<LmPluginMetaAnalysis> {
    let state = Box::new(MetaState {
        name: CString::new(pass.name()).map_err(|_| interior_nul("name"))?,
        provides: CString::new(M::PROVIDES).map_err(|_| interior_nul("provides"))?,
        result_type_tag: CString::new(<M::Output as AnalysisResult>::static_type_tag())
            .map_err(|_| interior_nul("result type tag"))?,
        pass,
    });
    Ok(LmPluginMetaAnalysis {
        user_data: Box::into_raw(state) as *mut c_void,
        // A meta-analysis is not reversible, so its id is never used to route reversal; reuse the
        // name (records identify meta passes by name).
        id: meta_trampoline_name::<M>,
        name: meta_trampoline_name::<M>,
        provides: meta_trampoline_provides::<M>,
        result_type_tag: meta_trampoline_result_tag::<M>,
        run: meta_trampoline_run::<M>,
        free: meta_trampoline_free::<M>,
    })
}

/// Builds the C-ABI descriptor for a plugin **control-flow** pass. luna-model takes ownership of the
/// returned descriptor's `user_data` and releases it when the pass is dropped. Authors rarely call
/// this directly — a driver passes its result to the host to run the pass in a pipeline.
pub fn descriptor_control_flow<C: ControlFlowPass>(pass: C) -> crate::Result<LmPluginControlFlow> {
    let state = Box::new(ControlFlowState {
        // A control-flow pass is identified by name (records reference no separate id); reuse it.
        name: CString::new(pass.name()).map_err(|_| interior_nul("name"))?,
        pass,
    });
    Ok(LmPluginControlFlow {
        user_data: Box::into_raw(state) as *mut c_void,
        id: control_flow_trampoline_name::<C>,
        name: control_flow_trampoline_name::<C>,
        requires_analyses: control_flow_trampoline_requires::<C>,
        provides: control_flow_trampoline_provides::<C>,
        invalidates: control_flow_trampoline_invalidates::<C>,
        run: control_flow_trampoline_run::<C>,
        free: control_flow_trampoline_free::<C>,
    })
}

/// Registers a plugin transformation pass's reversal handler with the host, keyed by
/// [`Reversible::ID`]. Call once per pass type (typically from a plugin's registration hook) so the
/// host can reverse later.
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

/// Registers a plugin **composite** pass's reversal handler with the host (composites are
/// reversible too). Mirrors [`register_backward`] but for the composite kind.
pub fn register_composite_backward<C: CompositePass>() -> crate::Result<()> {
    // Safety: `trampoline_backward::<C>` is a valid reversal callback; it needs no `user_data` and
    // dispatches to the associated `C::backward` (from the `Reversible` super-trait).
    unsafe {
        register_plugin_backward(
            LmPassKind::Composite,
            C::ID,
            trampoline_backward::<C>,
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
        kind: None,
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
        Ok(artifact) => {
            let bytes = artifact.encode_to_vec();
            unsafe { sink(sink_ctx, bytes.as_ptr(), bytes.len()) };
            LmStatus::Ok
        }
        Err(e) => crate::error::report(&e),
    }
}

unsafe extern "C" fn trampoline_backward<P: Reversible>(
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
    let artifact = match <P::Artifact as Message>::decode(bytes) {
        Ok(artifact) => artifact,
        Err(_) => return LmStatus::Deserialization,
    };
    match P::backward(&artifact, solution) {
        Ok(result) => {
            unsafe { *out_solution = result.into_raw_owned() };
            LmStatus::Ok
        }
        Err(e) => crate::error::report(&e),
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
    unsafe {
        (*(user_data as *const AnalysisState<A>))
            .result_type_tag
            .as_ptr()
    }
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
        Ok(output) => {
            let bytes = output.encode_to_vec();
            unsafe { sink(sink_ctx, bytes.as_ptr(), bytes.len()) };
            LmStatus::Ok
        }
        Err(e) => crate::error::report(&e),
    }
}

unsafe extern "C" fn analysis_trampoline_free<A: AnalysisPass>(user_data: *mut c_void) {
    drop(unsafe { Box::from_raw(user_data as *mut AnalysisState<A>) });
}

/// Boxed plugin state behind a composite descriptor's `user_data`, caching the id/name/provides/
/// result-tag C strings so their trampolines can hand out stable borrowed pointers.
struct CompositeState<C: CompositePass> {
    pass: C,
    id: CString,
    name: CString,
    provides: CString,
    result_type_tag: CString,
}

unsafe extern "C" fn composite_trampoline_id<C: CompositePass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const CompositeState<C>)).id.as_ptr() }
}

unsafe extern "C" fn composite_trampoline_name<C: CompositePass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const CompositeState<C>)).name.as_ptr() }
}

unsafe extern "C" fn composite_trampoline_provides<C: CompositePass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const CompositeState<C>)).provides.as_ptr() }
}

unsafe extern "C" fn composite_trampoline_result_tag<C: CompositePass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe {
        (*(user_data as *const CompositeState<C>))
            .result_type_tag
            .as_ptr()
    }
}

unsafe extern "C" fn composite_trampoline_requires<C: CompositePass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const CompositeState<C>) };
    unsafe { emit_strings(state.pass.requires(), sink, ctx) };
}

unsafe extern "C" fn composite_trampoline_invalidates<C: CompositePass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const CompositeState<C>) };
    unsafe { emit_strings(state.pass.invalidates(), sink, ctx) };
}

unsafe extern "C" fn composite_trampoline_forward<C: CompositePass>(
    user_data: *mut c_void,
    model: *mut LmModel,
    ctx: *const LmPassContext,
    artifact_sink: LmByteSink,
    artifact_sink_ctx: *mut c_void,
    result_sink: LmByteSink,
    result_sink_ctx: *mut c_void,
) -> LmStatus {
    let state = unsafe { &*(user_data as *const CompositeState<C>) };
    // Borrow the lent handle, run the safe `forward`, then relinquish it without freeing.
    let mut model = match unsafe { adopt::<Model>(model) } {
        Ok(model) => model,
        Err(_) => return LmStatus::NullPointer,
    };
    let context = PassContext::from_raw(ctx);
    let result = state.pass.forward(&mut model, &context);
    let _ = model.into_raw_owned();
    match result {
        Ok((artifact, output)) => {
            // Emit the two payloads independently: inverse artifact, then analysis result.
            let artifact_bytes = artifact.encode_to_vec();
            unsafe {
                artifact_sink(
                    artifact_sink_ctx,
                    artifact_bytes.as_ptr(),
                    artifact_bytes.len(),
                )
            };
            let result_bytes = output.encode_to_vec();
            unsafe { result_sink(result_sink_ctx, result_bytes.as_ptr(), result_bytes.len()) };
            LmStatus::Ok
        }
        Err(e) => crate::error::report(&e),
    }
}

unsafe extern "C" fn composite_trampoline_free<C: CompositePass>(user_data: *mut c_void) {
    drop(unsafe { Box::from_raw(user_data as *mut CompositeState<C>) });
}

/// Boxed plugin state behind a meta-analysis descriptor's `user_data`, caching the name/provides/
/// result-tag C strings so their trampolines can hand out stable borrowed pointers.
struct MetaState<M: MetaAnalysisPass> {
    pass: M,
    name: CString,
    provides: CString,
    result_type_tag: CString,
}

unsafe extern "C" fn meta_trampoline_name<M: MetaAnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const MetaState<M>)).name.as_ptr() }
}

unsafe extern "C" fn meta_trampoline_provides<M: MetaAnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const MetaState<M>)).provides.as_ptr() }
}

unsafe extern "C" fn meta_trampoline_result_tag<M: MetaAnalysisPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe {
        (*(user_data as *const MetaState<M>))
            .result_type_tag
            .as_ptr()
    }
}

unsafe extern "C" fn meta_trampoline_run<M: MetaAnalysisPass>(
    user_data: *mut c_void,
    steps: *const LmStepList,
    sink: LmByteSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    let state = unsafe { &*(user_data as *const MetaState<M>) };
    let step_list = StepList::from_raw(steps);
    match state.pass.run(&step_list) {
        Ok(output) => {
            let bytes = output.encode_to_vec();
            unsafe { sink(sink_ctx, bytes.as_ptr(), bytes.len()) };
            LmStatus::Ok
        }
        Err(e) => crate::error::report(&e),
    }
}

unsafe extern "C" fn meta_trampoline_free<M: MetaAnalysisPass>(user_data: *mut c_void) {
    drop(unsafe { Box::from_raw(user_data as *mut MetaState<M>) });
}

/// Boxed plugin state behind a control-flow descriptor's `user_data`, caching the name C string so
/// its trampolines can hand out a stable borrowed pointer.
struct ControlFlowState<C: ControlFlowPass> {
    pass: C,
    name: CString,
}

unsafe extern "C" fn control_flow_trampoline_name<C: ControlFlowPass>(
    user_data: *mut c_void,
) -> *const c_char {
    unsafe { (*(user_data as *const ControlFlowState<C>)).name.as_ptr() }
}

unsafe extern "C" fn control_flow_trampoline_requires<C: ControlFlowPass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const ControlFlowState<C>) };
    unsafe { emit_strings(state.pass.requires(), sink, ctx) };
}

unsafe extern "C" fn control_flow_trampoline_provides<C: ControlFlowPass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const ControlFlowState<C>) };
    unsafe { emit_strings(state.pass.provides(), sink, ctx) };
}

unsafe extern "C" fn control_flow_trampoline_invalidates<C: ControlFlowPass>(
    user_data: *mut c_void,
    sink: LmStrSink,
    ctx: *mut c_void,
) {
    let state = unsafe { &*(user_data as *const ControlFlowState<C>) };
    unsafe { emit_strings(state.pass.invalidates(), sink, ctx) };
}

unsafe extern "C" fn control_flow_trampoline_run<C: ControlFlowPass>(
    user_data: *mut c_void,
    model: *const LmModel,
    ctx: *const LmPassContext,
    out_plan: *mut *mut LmControlFlowPlan,
) -> LmStatus {
    let state = unsafe { &*(user_data as *const ControlFlowState<C>) };
    // Borrow the lent read-only handle, run the safe `run`, then relinquish it without freeing.
    let model = match unsafe { adopt::<Model>(model as *mut LmModel) } {
        Ok(model) => model,
        Err(_) => return LmStatus::NullPointer,
    };
    let context = PassContext::from_raw(ctx);
    let result = state.pass.run(&model, &context);
    let _ = model.into_raw_owned();
    match result {
        Ok(plan) => {
            // Hand ownership of the built plan to the host through the out-parameter.
            unsafe { *out_plan = plan.into_raw() };
            LmStatus::Ok
        }
        Err(e) => crate::error::report(&e),
    }
}

unsafe extern "C" fn control_flow_trampoline_free<C: ControlFlowPass>(user_data: *mut c_void) {
    drop(unsafe { Box::from_raw(user_data as *mut ControlFlowState<C>) });
}
