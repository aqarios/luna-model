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
    /// Emits the analysis names this pass requires, one per `sink` call.
    pub requires:
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
