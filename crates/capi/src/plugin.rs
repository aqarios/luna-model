use std::ffi::{c_char, c_void};

use crate::{LmModel, LmSol, LmStatus};

/// The kind of transpiler pass a plugin registers.
///
/// C-ABI-stable selector for the five `lunamodel-transpiler` pass traits.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmPassKind {
    Transformation = 0,
    Analysis = 1,
    Composite = 2,
    ControlFlow = 3,
    MetaAnalysis = 4,
    /// A nested pipeline step. Not a plugin-registrable kind; reported by `step_list_kind` when a
    /// meta-analysis introspects a step that is itself a sub-pipeline.
    Pipeline = 5,
}

/// Opaque handle to the per-pass context (`PassContext`) lent to a plugin during
/// `forward`/`run`. Borrowed for the duration of the call only.
/// The plugin must not retain or free it.
///
/// cbindgen::opaque
#[repr(C)]
pub struct LmPassContext {
    _private: [u8; 0],
}

/// Opaque, read-only view of the pipeline steps following a meta-analysis pass, lent to it during
/// `run`. Introspected through the `step_list_*` API-table entries (length, and per-index kind /
/// name / requires / provides / invalidates). Borrowed for the duration of the call only — the
/// plugin must not retain or free it.
///
/// cbindgen::opaque
#[repr(C)]
pub struct LmStepList {
    _private: [u8; 0],
}

/// Callback luna-model uses to collect a list of string produced by a plugin (e.g.
/// `requires`/`invalidates`) without transferring ownership across the ABI. The plugin calls
/// `sink(ctx, s)` once per NUL-termianted string. luna-model copies each immediately, so `s` need
/// only stay valid for the call.
pub type LmStrSink = unsafe extern "C" fn(ctx: *mut c_void, s: *const c_char);

/// Callback luna-model uses to collect a byte payload form a plugin (e.g., a serialized artifact)
/// without transferring ownership across the ABI. The plugin calls `sink(ctx, data, len)`.
/// luna-model copies the bytes immediately.
pub type LmByteSink = unsafe extern "C" fn(ctx: *mut c_void, data: *const u8, len: usize);

/// Reversal handler for a plugin transformation/composite pass, registered by stable pass id via
/// `register_plugin_backward`.
///
/// Mirrors the native by-value `backward`: ownership of `solution` is transferred IN to the plugin,
/// and the plugin writes the produced (mapped-back) owned solution OUT through `out_solution`. This
/// consume/produce shape lets reversal replace the solution entirely, not just mutate it. Invoked
/// later — possibly after the pass instance is gone or a `TransformationRecord` was reloaded — so it
/// must be self-contained given the artifact bytes plus `user_data`. On any non-`Ok` return,
/// `out_solution` is not read and the plugin owns (and must release) the incoming `solution`.
pub type LmBackwardFn = unsafe extern "C" fn(
    user_data: *mut c_void,
    artifact: *const u8,
    artifact_len: usize,
    solution: *mut LmSol,
    out_solution: *mut *mut LmSol,
) -> LmStatus;

/// Descriptor for a plugin **transformation** pass.
///
/// Filled by an extension and handed to luna-model via `lm_add_plugin_transform`. luna-model copies
/// the struct takes ownership of `user_data`, and wraps it in a host-side pass the transpiler
/// treats exactly like a built-in one. All function pointers must be non-null.
///
/// The model crosses as a live borrowed handle: `forward` receives a `*mut LmModel` it mutates
/// through the ordinary `lm_*` API. Reversal is registered separately (see [`LmBackwardFn`]).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LmPluginTransform {
    /// Opaque plugin state. luna-model owns it once the pass is added and releases it via `free`.
    /// Passed back to every callback.
    pub user_data: *mut c_void,
    /// Stable id used to route reversal to the matching [`LmBackwardFn`]
    /// (e.g. `"myplugin::relabel"`). Return a borrowed string, copied immediately.
    pub id: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Human-readable pass name. Returns a borrowed string, copied immediately.
    pub name: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Emits the analysis names this pass requires, one per `sink` call. (Named `requires_analyses`,
    /// not `requires`, because `requires` is a reserved keyword in C++20 and this struct is used
    /// from C++ plugin authors.)
    pub requires_analyses:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Emits the analysis names this pass invalidates, one per `sink` call.
    pub invalidates:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Applies the transformation to the lent model and emits the inverse-artifact bytes via
    /// `sink`. Return non-`Ok` to abort the run.
    pub forward: unsafe extern "C" fn(
        user_data: *mut c_void,
        model: *mut LmModel,
        ctx: *const LmPassContext,
        sink: LmByteSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Releases `user_data`. Called once when luna-model drops the pass.
    pub free: unsafe extern "C" fn(user_data: *mut c_void),
}

/// Descriptor for a plugin **analysis** pass.
///
/// Filled by an extension and handed to luna-model, which copies it, takes ownership of `user_data`,
/// and wraps it in a host-side pass the transpiler runs like a built-in analysis. The analysis
/// reads the lent model and produces a serialized result deposited under `provides` in the run's
/// analysis store, where later passes — including *other* plugins — read it back by that name (via
/// `pass_context_get_analysis`). All function pointers must be non-null.
///
/// The model crosses as a live borrowed handle for reading only; unlike a transformation it is not
/// moved out and back.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LmPluginAnalysis {
    /// Opaque plugin state. luna-model owns it once the pass is added and releases it via `free`.
    pub user_data: *mut c_void,
    /// Stable pass id (e.g. `"myplugin::coeff-widths-analysis"`). Borrowed, copied immediately.
    pub id: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Human-readable pass name. Borrowed, copied immediately.
    pub name: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Name the result is stored under in the analysis store (e.g. `"myplugin::coeff-widths"`);
    /// consumers read it back with this name. Borrowed, copied immediately.
    pub provides: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Stable type tag of the produced result type, verified when a consumer decodes it. Borrowed,
    /// copied immediately.
    pub result_type_tag: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Emits the analysis names this pass requires, one per `sink` call. (Named `requires_analyses`,
    /// not `requires`, because `requires` is a reserved keyword in C++20 and this struct is used
    /// from C++ plugin authors.)
    pub requires_analyses:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Computes the result from the lent read-only model and emits its serialized bytes via `sink`.
    /// Return non-`Ok` to abort the run.
    pub run: unsafe extern "C" fn(
        user_data: *mut c_void,
        model: *const LmModel,
        ctx: *const LmPassContext,
        sink: LmByteSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Releases `user_data`. Called once when luna-model drops the pass.
    pub free: unsafe extern "C" fn(user_data: *mut c_void),
}

/// Descriptor for a plugin **composite** pass — a fused transformation + analysis.
///
/// A composite both mutates the model (like [`LmPluginTransform`]) and produces a serialized
/// analysis result deposited under `provides` (like [`LmPluginAnalysis`]), in a single forward.
/// Its `forward` therefore emits TWO payloads: the inverse-artifact bytes (through `artifact_sink`)
/// and the analysis-result bytes (through `result_sink`). Reversal is registered separately (see
/// [`LmBackwardFn`], with kind [`LmPassKind::Composite`]).
///
/// The model crosses as a live borrowed handle it mutates through the ordinary `lm_*` API, exactly
/// like a transformation. All function pointers must be non-null.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LmPluginComposite {
    /// Opaque plugin state. luna-model owns it once the pass is added and releases it via `free`.
    pub user_data: *mut c_void,
    /// Stable id used to route reversal to the matching [`LmBackwardFn`]. Borrowed, copied
    /// immediately.
    pub id: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Human-readable pass name. Borrowed, copied immediately.
    pub name: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Name the analysis result is stored under; consumers read it back with this name. Borrowed,
    /// copied immediately.
    pub provides: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Stable type tag of the produced result type, verified when a consumer decodes it. Borrowed,
    /// copied immediately.
    pub result_type_tag: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Emits the analysis names this pass requires, one per `sink` call. (Named `requires_analyses`,
    /// not `requires`, because `requires` is a reserved keyword in C++20 and this struct is used
    /// from C++ plugin authors.)
    pub requires_analyses:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Emits the analysis names this pass invalidates, one per `sink` call.
    pub invalidates:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Applies the transformation to the lent model, emitting the inverse-artifact bytes via
    /// `artifact_sink` and the analysis-result bytes via `result_sink`. Return non-`Ok` to abort
    /// the run.
    pub forward: unsafe extern "C" fn(
        user_data: *mut c_void,
        model: *mut LmModel,
        ctx: *const LmPassContext,
        artifact_sink: LmByteSink,
        artifact_sink_ctx: *mut c_void,
        result_sink: LmByteSink,
        result_sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Releases `user_data`. Called once when luna-model drops the pass.
    pub free: unsafe extern "C" fn(user_data: *mut c_void),
}

/// Descriptor for a plugin **meta-analysis** pass.
///
/// A meta-analysis computes a result *about the pipeline itself*: unlike an ordinary analysis it
/// reads no model, but instead introspects the steps that follow it (via the [`LmStepList`] lent to
/// `run`, through the `step_list_*` API-table entries) and deposits a serialized result under
/// `provides`, where later passes read it back. It is not reversible. All function pointers must be
/// non-null.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LmPluginMetaAnalysis {
    /// Opaque plugin state. luna-model owns it once the pass is added and releases it via `free`.
    pub user_data: *mut c_void,
    /// Stable pass id (e.g. `"myplugin::depth-meta"`). Borrowed, copied immediately.
    pub id: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Human-readable pass name. Borrowed, copied immediately.
    pub name: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Name the result is stored under; consumers read it back with this name. Borrowed, copied
    /// immediately.
    pub provides: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Stable type tag of the produced result type, verified when a consumer decodes it. Borrowed,
    /// copied immediately.
    pub result_type_tag: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Computes the result from the lent read-only view of the following pipeline steps and emits
    /// its serialized bytes via `sink`. Return non-`Ok` to abort the run.
    pub run: unsafe extern "C" fn(
        user_data: *mut c_void,
        steps: *const LmStepList,
        sink: LmByteSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Releases `user_data`. Called once when luna-model drops the pass.
    pub free: unsafe extern "C" fn(user_data: *mut c_void),
}

/// Opaque handle to a control-flow execution plan (`ControlFlowPlan`) built by a control-flow
/// plugin pass during `run`.
///
/// cbindgen::opaque
#[repr(C)]
pub struct LmControlFlowPlan {
    _private: [u8; 0],
}

/// Descriptor for a plugin **control-flow** pass.
///
/// Filled by an extension and handed to luna-model, which copies it, takes ownership of `user_data`,
/// and wraps it in a host-side pass the transpiler runs like a built-in control-flow pass. The
/// pass reads the lent read-only model and context, constructs an [`LmControlFlowPlan`], populates it
/// with steps via `control_flow_plan_add_*`, and hands ownership back through `out_plan`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LmPluginControlFlow {
    /// Opaque plugin state. luna-model owns it once the pass is added and releases it via `free`.
    pub user_data: *mut c_void,
    /// Stable pass id (e.g. `"myplugin::ifelse"`). Borrowed, copied immediately.
    pub id: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Human-readable pass name. Borrowed, copied immediately.
    pub name: unsafe extern "C" fn(user_data: *mut c_void) -> *const c_char,
    /// Emits the analysis names this pass requires, one per `sink` call.
    pub requires_analyses:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Emits the analysis names this pass provides, one per `sink` call.
    pub provides:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Emits the analysis names this pass invalidates, one per `sink` call.
    pub invalidates:
        unsafe extern "C" fn(user_data: *mut c_void, sink: LmStrSink, sink_ctx: *mut c_void),
    /// Evaluates control flow against the lent read-only model, constructing and returning a
    /// plan via `out_plan`. Return non-`Ok` to abort the run.
    pub run: unsafe extern "C" fn(
        user_data: *mut c_void,
        model: *const LmModel,
        ctx: *const LmPassContext,
        out_plan: *mut *mut LmControlFlowPlan,
    ) -> LmStatus,
    /// Releases `user_data`. Called once when luna-model drops the pass.
    pub free: unsafe extern "C" fn(user_data: *mut c_void),
}
