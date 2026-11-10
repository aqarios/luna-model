use std::{
    ffi::{c_char, c_void},
    mem::{offset_of, size_of},
};

use crate::plugin::{
    LmBackwardFn, LmByteSink, LmControlFlowPlan, LmModelParts, LmPassContext, LmPassKind,
    LmPluginAnalysis, LmPluginComposite, LmPluginControlFlow, LmPluginMetaAnalysis,
    LmPluginTransform, LmStepList, LmStrSink,
};

pub use super::{
    LmAssignment, LmBoolMapBuffer, LmBound, LmBounds, LmBuffer, LmCC, LmCmpBuffer, LmColumn,
    LmColumnBuffer, LmComparator, LmConstraint, LmCtype, LmCtypeBuffer, LmEnv, LmEnvId, LmError,
    LmExpr, LmF64Buffer, LmHigherOrder, LmHigherOrderItemBuffer, LmIndexMapItemBuffer,
    LmLazyBounds, LmLinear, LmLinearItemBuffer, LmModel, LmQuadratic, LmQuadraticItemBuffer,
    LmResultView, LmResultViewBuffer, LmSampleView, LmSampleViewBuffer, LmSense, LmSol,
    LmSolutionLookup, LmSpecs, LmStatus, LmStrBuffer, LmSubTimer, LmTimer, LmTiming, LmU32Buffer,
    LmValueSource, LmVar, LmVref, LmVrefBuffer, LmVtype, LmVtypeBuffer,
};

/// Stable, versioned table of LunaModel C API entry points.
///
/// Consumers obtaining this via the capsule must check [`Self::abi_version`]
/// against the `CAPI_ABI_VERSION` they were compiled against before calling.
///
/// Within one ABI version the table only grows: new entry points are appended at
/// the end, never inserted, reordered, removed, or changed (`ci/tools/check-abi.py`
/// enforces this against the last release). A consumer may therefore be built
/// against a larger table than the host provides; before calling an entry point
/// that may postdate the host, it checks [`capi_has!`](crate::capi_has).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LunaModelCApi {
    /// ABI version of this table (`CAPI_ABI_VERSION` at build time).
    pub abi_version: u32,
    /// Size in bytes of the host's table (`size_of::<LunaModelCApi>()` at the host's
    /// build). It occupies the padding after `abi_version`, so adding it kept every
    /// entry point's offset. `0` in tables of luna-model 0.8.0a1, which predate the
    /// field; those are [`CAPI_BASE_TABLE_SIZE`] bytes. Read it through
    /// [`LunaModelCApi::size`].
    pub struct_size: u32,
    pub version: extern "C" fn() -> *const c_char,
    pub buffer_free: unsafe extern "C" fn(LmBuffer),
    pub str_buffer_free: unsafe extern "C" fn(LmStrBuffer),
    pub vtype_buffer_free: unsafe extern "C" fn(LmVtypeBuffer),
    pub ctype_buffer_free: unsafe extern "C" fn(LmCtypeBuffer),
    pub cmp_buffer_free: unsafe extern "C" fn(LmCmpBuffer),
    pub vref_buffer_free: unsafe extern "C" fn(LmVrefBuffer),
    pub column_buffer_free: unsafe extern "C" fn(LmColumnBuffer),
    pub u32_buffer_free: unsafe extern "C" fn(LmU32Buffer),
    pub f64_buffer_free: unsafe extern "C" fn(LmF64Buffer),
    pub linear_item_buffer_free: unsafe extern "C" fn(LmLinearItemBuffer),
    pub quadratic_item_buffer_free: unsafe extern "C" fn(LmQuadraticItemBuffer),
    pub higher_order_item_buffer_free: unsafe extern "C" fn(LmHigherOrderItemBuffer),
    pub result_view_buffer_free: unsafe extern "C" fn(LmResultViewBuffer),
    pub sample_view_buffer_free: unsafe extern "C" fn(LmSampleViewBuffer),
    pub index_map_buffer_free: unsafe extern "C" fn(LmIndexMapItemBuffer),
    pub bool_map_buffer_free: unsafe extern "C" fn(LmBoolMapBuffer),
    // ///// //
    // MODEL //
    // ///// //
    pub model_free: unsafe extern "C" fn(*mut LmModel),
    // Model -- creation
    pub model_default: extern "C" fn() -> *mut LmModel,
    pub model_new:
        unsafe extern "C" fn(*const c_char, *const LmSense, *mut *mut LmModel) -> LmStatus,
    pub model_with_env: unsafe extern "C" fn(
        *const c_char,
        *const LmSense,
        *const LmEnv,
        *mut *mut LmModel,
    ) -> LmStatus,
    // Model -- general
    pub model_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_retain: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_deep_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_satisfies:
        unsafe extern "C" fn(*const LmModel, *const LmSpecs, *mut bool) -> LmStatus,
    pub model_add_var: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub model_add_var_with_fallback: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub model_eq: unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    pub model_equal_contents:
        unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    pub model_evaluate_solution:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *mut *mut LmSol) -> LmStatus,
    pub model_evaluate_solution_with_tol:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *const f64, *mut *mut LmSol) -> LmStatus,
    pub model_evaluate_sample: unsafe extern "C" fn(
        *const LmModel,
        *const LmSampleView,
        *const f64,
        *mut *mut LmResultView,
    ) -> LmStatus,
    pub model_violated_constraints: unsafe extern "C" fn(
        *const LmModel,
        *const LmSampleView,
        *const f64,
        *mut *mut LmCC,
    ) -> LmStatus,
    pub model_substitute:
        unsafe extern "C" fn(*mut LmModel, *const LmVref, *const LmExpr) -> LmStatus,
    // Model -- access
    pub model_name: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    pub model_sense: unsafe extern "C" fn(*const LmModel, *mut LmSense) -> LmStatus,
    pub model_env: unsafe extern "C" fn(*const LmModel, *mut *mut LmEnv) -> LmStatus,
    pub model_specs: unsafe extern "C" fn(*const LmModel, *mut *mut LmSpecs) -> LmStatus,
    pub model_vtypes: unsafe extern "C" fn(*const LmModel, *mut LmVtypeBuffer) -> LmStatus,
    pub model_num_variables: unsafe extern "C" fn(*const LmModel, *mut usize) -> LmStatus,
    pub model_vars: unsafe extern "C" fn(*const LmModel, *mut LmVrefBuffer) -> LmStatus,
    pub model_var:
        unsafe extern "C" fn(*const LmModel, *const c_char, *mut *mut LmVref) -> LmStatus,
    pub model_objective: unsafe extern "C" fn(*const LmModel, *mut *mut LmExpr) -> LmStatus,
    pub model_constraints: unsafe extern "C" fn(*const LmModel, *mut *mut LmCC) -> LmStatus,
    // Model -- setter
    pub model_set_name: unsafe extern "C" fn(*mut LmModel, *const c_char) -> LmStatus,
    pub model_set_sense: unsafe extern "C" fn(*mut LmModel, LmSense) -> LmStatus,
    pub model_set_objective:
        unsafe extern "C" fn(*mut LmModel, *const LmExpr, *const LmSense) -> LmStatus,
    pub model_set_constraints: unsafe extern "C" fn(*mut LmModel, *const LmCC) -> LmStatus,
    // Model -- serializer
    pub model_serialize: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    pub model_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmModel) -> LmStatus,
    // /////////// //
    // ENVIRONMENT //
    // /////////// //
    pub env_free: unsafe extern "C" fn(*mut LmEnv),
    pub env_default: extern "C" fn() -> *mut LmEnv,
    pub env_new: unsafe extern "C" fn(
        *const *const LmVar,
        *const u32,
        usize,
        *const *const c_char,
        *const u32,
        usize,
        u32,
        *mut *mut LmEnv,
    ) -> LmStatus,
    // Env -- general
    pub env_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    pub env_deep_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    pub env_eq: unsafe extern "C" fn(*const LmEnv, *const LmEnv, *mut bool) -> LmStatus,
    pub env_equal_contents: unsafe extern "C" fn(*const LmEnv, *const LmEnv, *mut bool) -> LmStatus,
    // Env -- access
    pub env_len: unsafe extern "C" fn(*const LmEnv, *mut usize) -> LmStatus,
    pub env_is_empty: unsafe extern "C" fn(*const LmEnv, *mut bool) -> LmStatus,
    pub env_vars: unsafe extern "C" fn(*const LmEnv, *mut LmVrefBuffer) -> LmStatus,
    pub env_id: unsafe extern "C" fn(*const LmEnv, *mut LmEnvId) -> LmStatus,
    pub env_lookup: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut *mut LmVref) -> LmStatus,
    pub env_get: unsafe extern "C" fn(*const LmEnv, u32, *mut *mut LmVref) -> LmStatus,
    pub env_contains: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut bool) -> LmStatus,
    pub env_vtype_of: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut LmVtype) -> LmStatus,
    pub env_sort: unsafe extern "C" fn(
        *const LmEnv,
        *const *const c_char,
        usize,
        *mut LmStrBuffer,
    ) -> LmStatus,
    pub env_next_idx: unsafe extern "C" fn(*const LmEnv, *mut u32) -> LmStatus,
    // Env -- modification
    pub env_insert: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub env_insert_with_fallback: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *const u64,
        usize,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub env_insert_inverted:
        unsafe extern "C" fn(*mut LmEnv, *const LmVref, *mut *mut LmVref) -> LmStatus,
    pub env_remove: unsafe extern "C" fn(*mut LmEnv, *const LmVref) -> LmStatus,
    pub env_set_inverted: unsafe extern "C" fn(*mut LmEnv, u32, bool, u32) -> LmStatus,
    // Env -- serializer
    pub env_serialize: unsafe extern "C" fn(*const LmEnv, *mut LmBuffer) -> LmStatus,
    pub env_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmEnv) -> LmStatus,
    // ////// //
    // COLUMN //
    // ////// //
    pub col_free: unsafe extern "C" fn(*mut LmColumn),
    // Column -- creation
    pub col_new: unsafe extern "C" fn(LmVtype, *const f64, usize, *mut *mut LmColumn) -> LmStatus,
    pub col_with: unsafe extern "C" fn(LmAssignment, *mut *mut LmColumn) -> LmStatus,
    // Column -- general
    pub col_clone: unsafe extern "C" fn(*const LmColumn, *mut *mut LmColumn) -> LmStatus,
    pub col_eq: unsafe extern "C" fn(*const LmColumn, *const LmColumn, *mut bool) -> LmStatus,
    pub col_filter_by_mask:
        unsafe extern "C" fn(*const LmColumn, *const bool, usize, *mut *mut LmColumn) -> LmStatus,
    pub col_extract: unsafe extern "C" fn(*const LmColumn, usize, *mut *mut LmColumn) -> LmStatus,
    // Column -- access
    pub col_len: unsafe extern "C" fn(*const LmColumn, *mut usize) -> LmStatus,
    pub col_is_empty: unsafe extern "C" fn(*const LmColumn, *mut bool) -> LmStatus,
    pub col_vtype: unsafe extern "C" fn(*const LmColumn, *mut LmVtype) -> LmStatus,
    pub col_assignment: unsafe extern "C" fn(*const LmColumn, usize, *mut LmAssignment) -> LmStatus,
    pub col_values: unsafe extern "C" fn(*const LmColumn, *mut *const f64, *mut usize) -> LmStatus,
    pub col_index: unsafe extern "C" fn(*const LmColumn, usize, *mut f64) -> LmStatus,
    // Column -- modification
    pub col_push: unsafe extern "C" fn(*mut LmColumn, LmAssignment) -> LmStatus,
    pub col_try_push: unsafe extern "C" fn(*mut LmColumn, f64, *const f64) -> LmStatus,
    pub col_remove: unsafe extern "C" fn(*mut LmColumn, usize) -> LmStatus,
    // /////////// //
    // RESULT_VIEW //
    // /////////// //
    pub result_view_free: unsafe extern "C" fn(*mut LmResultView),
    pub result_view_counts: unsafe extern "C" fn(*const LmResultView, *mut usize) -> LmStatus,
    pub result_view_obj_value: unsafe extern "C" fn(*const LmResultView, *mut f64) -> LmStatus,
    pub result_view_raw_energy: unsafe extern "C" fn(*const LmResultView, *mut f64) -> LmStatus,
    pub result_view_constraints:
        unsafe extern "C" fn(*const LmResultView, *mut LmBoolMapBuffer) -> LmStatus,
    pub result_view_variable_bounds:
        unsafe extern "C" fn(*const LmResultView, *mut LmBoolMapBuffer) -> LmStatus,
    pub result_view_feasible: unsafe extern "C" fn(*const LmResultView, *mut bool) -> LmStatus,
    pub result_view_sample:
        unsafe extern "C" fn(*const LmResultView, *mut *mut LmSampleView) -> LmStatus,
    // /////////// //
    // SAMPLE_VIEW //
    // /////////// //
    pub sample_view_free: unsafe extern "C" fn(*mut LmSampleView),
    pub sample_view_index: unsafe extern "C" fn(*const LmSampleView, *mut usize) -> LmStatus,
    pub sample_view_variable_names:
        unsafe extern "C" fn(*const LmSampleView, *mut LmStrBuffer) -> LmStatus,
    pub sample_view_assignment:
        unsafe extern "C" fn(*const LmSampleView, *const c_char, *mut LmAssignment) -> LmStatus,
    pub sample_view_assignment_idx:
        unsafe extern "C" fn(*const LmSampleView, usize, *mut LmAssignment) -> LmStatus,
    // //////// //
    // SOLUTION //
    // //////// //
    pub sol_free: unsafe extern "C" fn(*mut LmSol),
    pub sol_default: extern "C" fn() -> *mut LmSol,
    pub sol_with_sense: extern "C" fn(LmSense) -> *mut LmSol,
    pub sol_with_capacity: extern "C" fn(usize) -> *mut LmSol,
    pub sol_from_env: unsafe extern "C" fn(*const LmEnv, *mut *mut LmSol) -> LmStatus,
    pub sol_from_random: unsafe extern "C" fn(
        usize,
        *const u64,
        *const LmEnv,
        *const LmModel,
        *const LmSense,
        *const f64,
        *mut *mut LmSol,
    ) -> LmStatus,
    // Sol -- general
    pub sol_clone: unsafe extern "C" fn(*const LmSol, *mut *mut LmSol) -> LmStatus,
    pub sol_eq: unsafe extern "C" fn(*const LmSol, *const LmSol, *mut bool) -> LmStatus,
    pub sol_equal_contents: unsafe extern "C" fn(*const LmSol, *const LmSol, *mut bool) -> LmStatus,
    pub sol_aggregate: unsafe extern "C" fn(*mut LmSol) -> LmStatus,
    pub sol_merge_many: unsafe extern "C" fn(
        *const *const LmSol,
        usize,
        *const LmModel,
        *const f64,
        *mut *mut LmSol,
    ) -> LmStatus,
    pub sol_filter_by_mask:
        unsafe extern "C" fn(*const LmSol, *const bool, usize, *mut *mut LmSol) -> LmStatus,
    // Sol -- properties
    pub sol_samples: unsafe extern "C" fn(*const LmSol, *mut LmIndexMapItemBuffer) -> LmStatus,
    pub sol_column:
        unsafe extern "C" fn(*const LmSol, *const c_char, *mut *mut LmColumn) -> LmStatus,
    pub sol_counts: unsafe extern "C" fn(*const LmSol, *mut *const usize, *mut usize) -> LmStatus,
    pub sol_raw_energies:
        unsafe extern "C" fn(*const LmSol, *mut *const f64, *mut usize) -> LmStatus,
    pub sol_obj_values: unsafe extern "C" fn(*const LmSol, *mut *const f64, *mut usize) -> LmStatus,
    pub sol_feasible: unsafe extern "C" fn(*const LmSol, *mut *const bool, *mut usize) -> LmStatus,
    pub sol_constraints: unsafe extern "C" fn(*const LmSol, *mut LmBoolMapBuffer) -> LmStatus,
    pub sol_variable_bounds: unsafe extern "C" fn(*const LmSol, *mut LmBoolMapBuffer) -> LmStatus,
    pub sol_timing: unsafe extern "C" fn(*const LmSol, *mut *mut LmTiming) -> LmStatus,
    pub sol_sense: unsafe extern "C" fn(*const LmSol, *mut LmSense) -> LmStatus,
    // Sol -- access
    pub sol_n_samples: unsafe extern "C" fn(*const LmSol, *mut usize) -> LmStatus,
    pub sol_len: unsafe extern "C" fn(*const LmSol, *mut usize) -> LmStatus,
    pub sol_variable_names: unsafe extern "C" fn(*const LmSol, *mut LmStrBuffer) -> LmStatus,
    pub sol_is_empty: unsafe extern "C" fn(*const LmSol, *mut bool) -> LmStatus,
    pub sol_sample_len: unsafe extern "C" fn(*const LmSol, *mut usize) -> LmStatus,
    pub sol_value: unsafe extern "C" fn(*const LmSol, usize, *const c_char, *mut f64) -> LmStatus,
    pub sol_assignment:
        unsafe extern "C" fn(*const LmSol, usize, *const c_char, *mut LmAssignment) -> LmStatus,
    pub sol_assignment_idx:
        unsafe extern "C" fn(*const LmSol, usize, usize, *mut LmAssignment) -> LmStatus,
    pub sol_result: unsafe extern "C" fn(*const LmSol, usize, *mut *mut LmResultView) -> LmStatus,
    pub sol_results: unsafe extern "C" fn(*const LmSol, *mut LmResultViewBuffer) -> LmStatus,
    pub sol_sample_view:
        unsafe extern "C" fn(*const LmSol, usize, *mut *mut LmSampleView) -> LmStatus,
    pub sol_sample_views: unsafe extern "C" fn(*const LmSol, *mut LmSampleViewBuffer) -> LmStatus,
    pub sol_best: unsafe extern "C" fn(*const LmSol, *mut LmResultViewBuffer) -> LmStatus,
    pub sol_extract: unsafe extern "C" fn(*const LmSol, usize, *mut *mut LmSol) -> LmStatus,
    // Sol -- modification
    pub sol_push: unsafe extern "C" fn(
        *mut LmSol,
        *const *const c_char,
        *const LmAssignment,
        usize,
        usize,
        *const f64,
    ) -> LmStatus,
    pub sol_add_empty_binary: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_empty_spin: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_empty_integer: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_empty_real: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_binary:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize, *const f64) -> LmStatus,
    pub sol_add_spin:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize, *const f64) -> LmStatus,
    pub sol_add_integer:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize, *const f64) -> LmStatus,
    pub sol_add_real:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize) -> LmStatus,
    pub sol_add_col: unsafe extern "C" fn(
        *mut LmSol,
        LmVtype,
        *const c_char,
        *const f64,
        usize,
        *const f64,
    ) -> LmStatus,
    pub sol_remove_col:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *mut *mut LmColumn) -> LmStatus,
    pub sol_remove_cols: unsafe extern "C" fn(
        *mut LmSol,
        *const *const c_char,
        usize,
        *mut LmColumnBuffer,
    ) -> LmStatus,
    pub sol_set_counts: unsafe extern "C" fn(*mut LmSol, *const usize, usize) -> LmStatus,
    pub sol_set_timing: unsafe extern "C" fn(*mut LmSol, *const LmTiming) -> LmStatus,
    pub sol_set_sense: unsafe extern "C" fn(*mut LmSol, LmSense) -> LmStatus,
    // Sol -- setters for evaluation results (null pointer clears to `None`;
    // non-null with len 0 is an explicit empty `Some(vec![])`)
    pub sol_set_raw_energies: unsafe extern "C" fn(*mut LmSol, *const f64, usize) -> LmStatus,
    pub sol_set_obj_values: unsafe extern "C" fn(*mut LmSol, *const f64, usize) -> LmStatus,
    pub sol_set_feasible: unsafe extern "C" fn(*mut LmSol, *const bool, usize) -> LmStatus,
    // Sol -- convenience
    pub sol_cvar:
        unsafe extern "C" fn(*const LmSol, f64, *const LmValueSource, *mut f64) -> LmStatus,
    pub sol_expectation_value:
        unsafe extern "C" fn(*const LmSol, *const LmValueSource, *mut f64) -> LmStatus,
    pub sol_temperature_weighted:
        unsafe extern "C" fn(*const LmSol, f64, *const LmValueSource, *mut f64) -> LmStatus,
    pub sol_feasibility_ratio: unsafe extern "C" fn(*const LmSol, *mut f64) -> LmStatus,
    pub sol_highest_constraint_violations:
        unsafe extern "C" fn(*const LmSol, *mut LmBuffer) -> LmStatus,
    // Sol -- serializer
    pub sol_serialize: unsafe extern "C" fn(*const LmSol, *mut LmBuffer) -> LmStatus,
    pub sol_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmSol) -> LmStatus,
    // /////////////// //
    // SOLUTION LOOKUP //
    // /////////////// //
    //
    // A bulk per-sample lookup table, avoiding one ABI round-trip per value in
    // hot per-sample loops (e.g. samplers). Built once from an env + solution,
    // updated in place per sample row, and written back on demand.
    pub sol_lookup_free: unsafe extern "C" fn(*mut LmSolutionLookup),
    pub sol_lookup_new:
        unsafe extern "C" fn(*const LmEnv, *const LmSol, *mut *mut LmSolutionLookup) -> LmStatus,
    pub sol_lookup_update:
        unsafe extern "C" fn(*mut LmSolutionLookup, *const LmSampleView) -> LmStatus,
    pub sol_lookup_values:
        unsafe extern "C" fn(*const LmSolutionLookup, *mut *const f64, *mut usize) -> LmStatus,
    pub sol_lookup_write_sol:
        unsafe extern "C" fn(*mut LmSolutionLookup, u32, *const f64, usize, *mut LmSol) -> LmStatus,
    // ////////// //
    // EXPRESSION //
    // ////////// //
    pub expr_free: unsafe extern "C" fn(*mut LmExpr),
    pub expr_default: extern "C" fn() -> *mut LmExpr,
    pub expr_new: unsafe extern "C" fn(*const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_new_constant: unsafe extern "C" fn(*const LmEnv, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_double: unsafe extern "C" fn(f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_size_t: unsafe extern "C" fn(usize, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_linear: unsafe extern "C" fn(*const LmLinear, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_quadratic: unsafe extern "C" fn(*const LmQuadratic, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_higher_order:
        unsafe extern "C" fn(*const LmHigherOrder, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_dense_quadratic: unsafe extern "C" fn(
        *const f64,
        usize,
        usize,
        *const f64,
        *const LmEnv,
        *mut *mut LmExpr,
    ) -> LmStatus,
    pub expr_clone: unsafe extern "C" fn(*const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_deep_clone:
        unsafe extern "C" fn(*const LmExpr, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_deep_clone_many:
        unsafe extern "C" fn(*const *const LmExpr, usize, *mut *mut LmExpr) -> LmStatus,
    pub expr_evaluate_sample_quick:
        unsafe extern "C" fn(*const LmExpr, *const f64, usize, *mut f64) -> LmStatus,
    pub expr_evaluate_sample: unsafe extern "C" fn(
        *const LmExpr,
        *const *const c_char,
        *const f64,
        usize,
        *mut f64,
    ) -> LmStatus,
    pub expr_evaluate:
        unsafe extern "C" fn(*const LmExpr, *const LmSol, *mut LmF64Buffer) -> LmStatus,
    pub expr_serialize: unsafe extern "C" fn(*const LmExpr, *mut LmBuffer) -> LmStatus,
    pub expr_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_eq: unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    pub expr_equal_contents:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    pub expr_env: unsafe extern "C" fn(*const LmExpr, *mut *mut LmEnv) -> LmStatus,
    pub expr_offset: unsafe extern "C" fn(*const LmExpr, *mut f64) -> LmStatus,
    pub expr_linear: unsafe extern "C" fn(*const LmExpr, *mut *mut LmLinear) -> LmStatus,
    pub expr_quadratic: unsafe extern "C" fn(*const LmExpr, *mut *mut LmQuadratic) -> LmStatus,
    pub expr_higher_order: unsafe extern "C" fn(*const LmExpr, *mut *mut LmHigherOrder) -> LmStatus,
    pub expr_num_vars: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    pub expr_vtypes: unsafe extern "C" fn(*const LmExpr, *mut LmVtypeBuffer) -> LmStatus,
    pub expr_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    pub expr_complete_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    pub expr_contains: unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut bool) -> LmStatus,
    pub expr_degree: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    pub expr_is_constant: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_has_quadratic: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_has_higher_order: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_set_offset: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_set_linear: unsafe extern "C" fn(*mut LmExpr, *const LmLinear) -> LmStatus,
    pub expr_set_quadratic: unsafe extern "C" fn(*mut LmExpr, *const LmQuadratic) -> LmStatus,
    pub expr_set_higher_order: unsafe extern "C" fn(*mut LmExpr, *const LmHigherOrder) -> LmStatus,
    pub expr_add_linear_term: unsafe extern "C" fn(*mut LmExpr, u32, f64) -> LmStatus,
    pub expr_add_quadratic_term: unsafe extern "C" fn(*mut LmExpr, u32, u32, f64) -> LmStatus,
    pub expr_add_higher_order_term:
        unsafe extern "C" fn(*mut LmExpr, *const u32, usize, f64) -> LmStatus,
    pub expr_add_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_add_vref:
        unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut *mut LmExpr) -> LmStatus,
    pub expr_add_expr:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_sub_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_rsub_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus, // todo
    pub expr_sub_vref:
        unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut *mut LmExpr) -> LmStatus,
    pub expr_sub_expr:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_mul_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_mul_vref:
        unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut *mut LmExpr) -> LmStatus,
    pub expr_mul_expr:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_iadd_bias: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_iadd_vref: unsafe extern "C" fn(*mut LmExpr, *const LmVref) -> LmStatus,
    pub expr_iadd_expr: unsafe extern "C" fn(*mut LmExpr, *const LmExpr) -> LmStatus,
    pub expr_isub_bias: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_isub_vref: unsafe extern "C" fn(*mut LmExpr, *const LmVref) -> LmStatus,
    pub expr_isub_expr: unsafe extern "C" fn(*mut LmExpr, *const LmExpr) -> LmStatus,
    pub expr_imul_bias: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_imul_vref: unsafe extern "C" fn(*mut LmExpr, *const LmVref) -> LmStatus,
    pub expr_imul_expr: unsafe extern "C" fn(*mut LmExpr, *const LmExpr) -> LmStatus,
    pub expr_neg: unsafe extern "C" fn(*const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_pow: unsafe extern "C" fn(*const LmExpr, usize, *mut *mut LmExpr) -> LmStatus,
    pub expr_ipow: unsafe extern "C" fn(*mut LmExpr, usize) -> LmStatus,
    pub expr_separate: unsafe extern "C" fn(
        *const LmExpr,
        *const *const LmVref,
        usize,
        *mut *mut LmExpr,
        *mut *mut LmExpr,
    ) -> LmStatus,
    pub expr_substitute: unsafe extern "C" fn(
        *const LmExpr,
        *const LmVref,
        *const LmExpr,
        *mut *mut LmExpr,
    ) -> LmStatus,
    // ////////// //
    // Constraint //
    // ////////// //
    pub constr_free: unsafe extern "C" fn(*mut LmConstraint),
    pub constr_new: unsafe extern "C" fn(
        *const LmExpr,
        f64,
        LmComparator,
        *const c_char,
        *mut *mut LmConstraint,
    ) -> LmStatus,
    pub constr_clone: unsafe extern "C" fn(*const LmConstraint, *mut *mut LmConstraint) -> LmStatus,
    pub constr_deep_clone:
        unsafe extern "C" fn(*const LmConstraint, *const LmEnv, *mut *mut LmConstraint) -> LmStatus,
    pub constr_substitute:
        unsafe extern "C" fn(*mut LmConstraint, *const LmVref, *const LmExpr) -> LmStatus,
    pub constr_equal_contents:
        unsafe extern "C" fn(*const LmConstraint, *const LmConstraint, *mut bool) -> LmStatus,
    pub constr_eq:
        unsafe extern "C" fn(*const LmConstraint, *const LmConstraint, *mut bool) -> LmStatus,
    pub constr_evaluate_sample: unsafe extern "C" fn(
        *const LmConstraint,
        *const *const c_char,
        *const f64,
        usize,
        *const f64,
        *mut bool,
    ) -> LmStatus,
    pub constr_evaluate_sample_quick: unsafe extern "C" fn(
        *const LmConstraint,
        *const f64,
        usize,
        *const f64,
        *mut bool,
    ) -> LmStatus,
    pub constr_lhs: unsafe extern "C" fn(*const LmConstraint, *mut *mut LmExpr) -> LmStatus,
    pub constr_rhs: unsafe extern "C" fn(*const LmConstraint, *mut f64) -> LmStatus,
    pub constr_comparator: unsafe extern "C" fn(*const LmConstraint, *mut LmComparator) -> LmStatus,
    pub constr_name: unsafe extern "C" fn(*const LmConstraint, *mut LmBuffer) -> LmStatus,
    pub constr_auto_name: unsafe extern "C" fn(*const LmConstraint, *mut bool) -> LmStatus,
    pub constr_set_name: unsafe extern "C" fn(*mut LmConstraint, *const c_char) -> LmStatus,
    // //////////////////// //
    // ConstraintCollection //
    // //////////////////// //
    pub cc_free: unsafe extern "C" fn(*mut LmCC),
    pub cc_default: extern "C" fn() -> *mut LmCC,
    pub cc_new: extern "C" fn() -> *mut LmCC,
    pub cc_clone: unsafe extern "C" fn(*const LmCC, *mut *mut LmCC) -> LmStatus,
    pub cc_deep_clone: unsafe extern "C" fn(*const LmCC, *const LmEnv, *mut *mut LmCC) -> LmStatus,
    pub cc_eq: unsafe extern "C" fn(*const LmCC, *const LmCC, *mut bool) -> LmStatus,
    pub cc_equal_contents: unsafe extern "C" fn(*const LmCC, *const LmCC, *mut bool) -> LmStatus,
    pub cc_evaluate_sample: unsafe extern "C" fn(
        *const LmCC,
        *const *const c_char,
        *const f64,
        usize,
        *const f64,
        *mut LmBoolMapBuffer,
    ) -> LmStatus,
    pub cc_evaluate_sample_quick: unsafe extern "C" fn(
        *const LmCC,
        *const f64,
        usize,
        *const f64,
        *mut LmBoolMapBuffer,
    ) -> LmStatus,
    pub cc_vtypes: unsafe extern "C" fn(*const LmCC, *mut LmVtypeBuffer) -> LmStatus,
    pub cc_ctypes: unsafe extern "C" fn(*const LmCC, *mut LmCmpBuffer) -> LmStatus,
    pub cc_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    pub cc_complete_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    pub cc_is_empty: unsafe extern "C" fn(*const LmCC, *mut bool) -> LmStatus,
    pub cc_len: unsafe extern "C" fn(*const LmCC, *mut usize) -> LmStatus,
    pub cc_names: unsafe extern "C" fn(*const LmCC, *mut LmStrBuffer) -> LmStatus,
    pub cc_contains: unsafe extern "C" fn(*const LmCC, *const c_char, *mut bool) -> LmStatus,
    pub cc_get:
        unsafe extern "C" fn(*const LmCC, *const c_char, *mut *mut LmConstraint) -> LmStatus,
    pub cc_add_constraint: unsafe extern "C" fn(
        *mut LmCC,
        *const LmConstraint,
        *const c_char,
        *mut LmBuffer,
    ) -> LmStatus,
    pub cc_add_many_constraints: unsafe extern "C" fn(
        *mut LmCC,
        *const *const LmConstraint,
        *const *const c_char,
        usize,
        *mut LmStrBuffer,
    ) -> LmStatus,
    pub cc_from_map: unsafe extern "C" fn(
        *const *const c_char,
        *const *const LmConstraint,
        usize,
        *mut *mut LmCC,
    ) -> LmStatus,
    pub cc_add_collection:
        unsafe extern "C" fn(*mut LmCC, *const LmCC, *const c_char, *mut LmStrBuffer) -> LmStatus,
    pub cc_set_constraint:
        unsafe extern "C" fn(*mut LmCC, *const c_char, *const LmConstraint) -> LmStatus,
    pub cc_remove_constraint: unsafe extern "C" fn(*mut LmCC, *const c_char) -> LmStatus,
    pub cc_substitute: unsafe extern "C" fn(*mut LmCC, *const LmVref, *const LmExpr) -> LmStatus,
    pub cc_serialize: unsafe extern "C" fn(*const LmCC, *mut LmBuffer) -> LmStatus,
    pub cc_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmCC) -> LmStatus,
    // ////// //
    // VarRef //
    // ////// //
    pub vref_free: unsafe extern "C" fn(*mut LmVref),
    pub vref_new: unsafe extern "C" fn(u32, *const LmEnv, *mut *mut LmVref) -> LmStatus,
    pub vref_clone: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
    pub vref_id: unsafe extern "C" fn(*const LmVref, *mut u32) -> LmStatus,
    pub vref_check_living: unsafe extern "C" fn(*const LmVref, *mut bool) -> LmStatus,
    pub vref_name: unsafe extern "C" fn(*const LmVref, *mut LmBuffer) -> LmStatus,
    pub vref_vtype: unsafe extern "C" fn(*const LmVref, *mut LmVtype) -> LmStatus,
    pub vref_bounds: unsafe extern "C" fn(*const LmVref, *mut *mut LmBounds) -> LmStatus,
    pub vref_inverted: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
    pub vref_hash: unsafe extern "C" fn(*const LmVref, *mut u64) -> LmStatus,
    pub vref_eq: unsafe extern "C" fn(*const LmVref, *const LmVref, *mut bool) -> LmStatus,
    pub vref_to_var: unsafe extern "C" fn(*const LmVref, *mut *mut LmVar) -> LmStatus,
    pub vref_mul_vref:
        unsafe extern "C" fn(*const LmVref, *const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_mul_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_add_vref:
        unsafe extern "C" fn(*const LmVref, *const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_add_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_sub_vref:
        unsafe extern "C" fn(*const LmVref, *const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_sub_expr:
        unsafe extern "C" fn(*const LmVref, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub vref_sub_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_rsub_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_neg: unsafe extern "C" fn(*const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_not: unsafe extern "C" fn(*const LmVref, out: *mut *mut LmVref) -> LmStatus,
    pub vref_pow: unsafe extern "C" fn(*const LmVref, usize, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_eval: unsafe extern "C" fn(*const LmVref, f64, *const f64, out: *mut bool) -> LmStatus,
    // //////// //
    // Variable //
    // //////// //
    pub var_free: unsafe extern "C" fn(*mut LmVar),
    pub var_new: unsafe extern "C" fn(
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVar,
    ) -> LmStatus,
    pub var_clone: unsafe extern "C" fn(*const LmVar, *mut *mut LmVar) -> LmStatus,
    pub var_eq: unsafe extern "C" fn(*const LmVar, *const LmVar, *mut bool) -> LmStatus,
    pub var_name: unsafe extern "C" fn(*const LmVar, *mut LmBuffer) -> LmStatus,
    pub var_vtype: unsafe extern "C" fn(*const LmVar, *mut LmVtype) -> LmStatus,
    pub var_bounds: unsafe extern "C" fn(*const LmVar, *mut *mut LmBounds) -> LmStatus,
    pub var_inverted: unsafe extern "C" fn(*const LmVar, *mut u32) -> LmStatus,
    // ///// //
    // Specs //
    // ///// //
    pub specs_free: unsafe extern "C" fn(*mut LmSpecs),
    pub specs_new: unsafe extern "C" fn(
        *const LmSense,
        *const LmVtype,
        usize,
        *const LmCtype,
        usize,
        *const usize,
        *const usize,
        *const usize,
        *mut *mut LmSpecs,
    ) -> LmStatus,
    pub specs_clone: unsafe extern "C" fn(*const LmSpecs, *mut *mut LmSpecs) -> LmStatus,
    pub specs_satisfies:
        unsafe extern "C" fn(*const LmSpecs, *const LmSpecs, *mut bool) -> LmStatus,
    pub specs_diff: unsafe extern "C" fn(*const LmSpecs, *const LmSpecs, *mut LmBuffer) -> LmStatus,
    pub specs_sense: unsafe extern "C" fn(*const LmSpecs, *mut LmSense) -> LmStatus,
    pub specs_vtypes: unsafe extern "C" fn(*const LmSpecs, *mut LmVtypeBuffer) -> LmStatus,
    pub specs_ctypes: unsafe extern "C" fn(*const LmSpecs, *mut LmCtypeBuffer) -> LmStatus,
    pub specs_max_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_max_constraint_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_max_num_variables: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    // ///// //
    // Bound //
    // ///// //
    pub bound_is_bounded: unsafe extern "C" fn(*const LmBound, *mut bool) -> LmStatus,
    pub bound_value: unsafe extern "C" fn(*const LmBound, *mut f64) -> LmStatus,
    pub bound_eq: unsafe extern "C" fn(*const LmBound, *const LmBound, *mut bool) -> LmStatus,
    // ////// //
    // Bounds //
    // ////// //
    pub bounds_free: unsafe extern "C" fn(*mut LmBounds),
    pub bounds_new: unsafe extern "C" fn(LmBound, LmBound, *mut *mut LmBounds) -> LmStatus,
    pub bounds_binary: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_spin: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_integer: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_real: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_default_for: unsafe extern "C" fn(LmVtype, *mut *mut LmBounds) -> LmStatus,
    pub bounds_clone: unsafe extern "C" fn(*const LmBounds, *mut *mut LmBounds) -> LmStatus,
    pub bounds_eq: unsafe extern "C" fn(*const LmBounds, *const LmBounds, *mut bool) -> LmStatus,
    pub bounds_lower: unsafe extern "C" fn(*const LmBounds, *mut LmBound) -> LmStatus,
    pub bounds_upper: unsafe extern "C" fn(*const LmBounds, *mut LmBound) -> LmStatus,
    pub bounds_eval: unsafe extern "C" fn(*const LmBounds, f64, *const f64, *mut bool) -> LmStatus,
    // ////// //
    // LazyBounds //
    // ////// //
    pub lazy_bounds_free: unsafe extern "C" fn(*mut LmLazyBounds),
    pub lazy_bounds_default: extern "C" fn() -> *mut LmLazyBounds,
    pub lazy_bounds_new:
        unsafe extern "C" fn(*const LmBound, *const LmBound, *mut *mut LmLazyBounds) -> LmStatus,
    pub lazy_bounds_concretize:
        unsafe extern "C" fn(*const LmLazyBounds, LmVtype, *mut *mut LmBounds) -> LmStatus,
    pub lazy_bounds_clone:
        unsafe extern "C" fn(*const LmLazyBounds, *mut *mut LmLazyBounds) -> LmStatus,
    pub lazy_bounds_lower: unsafe extern "C" fn(*const LmLazyBounds, *mut LmBound) -> LmStatus,
    pub lazy_bounds_upper: unsafe extern "C" fn(*const LmLazyBounds, *mut LmBound) -> LmStatus,
    // ////// //
    // Linear //
    // ////// //
    pub linear_free: unsafe extern "C" fn(*mut LmLinear),
    pub linear_default: extern "C" fn() -> *mut LmLinear,
    pub linear_for_var: unsafe extern "C" fn(u32, f64, *mut *mut LmLinear) -> LmStatus,
    pub linear_for_vars: unsafe extern "C" fn(u32, f64, u32, f64, *mut *mut LmLinear) -> LmStatus,
    pub linear_clone: unsafe extern "C" fn(*const LmLinear, *mut *mut LmLinear) -> LmStatus,
    pub linear_eq: unsafe extern "C" fn(*const LmLinear, *const LmLinear, *mut bool) -> LmStatus,
    pub linear_len: unsafe extern "C" fn(*const LmLinear, *mut usize) -> LmStatus,
    pub linear_is_zero: unsafe extern "C" fn(*const LmLinear, *mut bool) -> LmStatus,
    pub linear_is_empty: unsafe extern "C" fn(*const LmLinear, *mut bool) -> LmStatus,
    pub linear_items: unsafe extern "C" fn(*const LmLinear, *mut LmLinearItemBuffer) -> LmStatus,
    pub linear_index: unsafe extern "C" fn(*const LmLinear, u32, *mut f64) -> LmStatus,
    pub linear_add_entry: unsafe extern "C" fn(*mut LmLinear, u32, f64) -> LmStatus,
    pub linear_add_linear: unsafe extern "C" fn(*mut LmLinear, *const LmLinear) -> LmStatus,
    pub linear_mul_bias: unsafe extern "C" fn(*mut LmLinear, f64) -> LmStatus,
    pub linear_set_bias: unsafe extern "C" fn(*mut LmLinear, u32, f64) -> LmStatus,
    pub linear_neg: unsafe extern "C" fn(*const LmLinear, *mut *mut LmLinear) -> LmStatus,
    // ///////// //
    // Quadratic //
    // ///////// //
    pub quadratic_free: unsafe extern "C" fn(*mut LmQuadratic),
    pub quadratic_default: extern "C" fn() -> *mut LmQuadratic,
    pub quadratic_clone:
        unsafe extern "C" fn(*const LmQuadratic, *mut *mut LmQuadratic) -> LmStatus,
    pub quadratic_eq:
        unsafe extern "C" fn(*const LmQuadratic, *const LmQuadratic, *mut bool) -> LmStatus,
    pub quadratic_len: unsafe extern "C" fn(*const LmQuadratic, *mut usize) -> LmStatus,
    pub quadratic_is_zero: unsafe extern "C" fn(*const LmQuadratic, *mut bool) -> LmStatus,
    pub quadratic_is_empty: unsafe extern "C" fn(*const LmQuadratic, *mut bool) -> LmStatus,
    pub quadratic_items:
        unsafe extern "C" fn(*const LmQuadratic, *mut LmQuadraticItemBuffer) -> LmStatus,
    pub quadratic_index: unsafe extern "C" fn(*const LmQuadratic, u32, u32, *mut f64) -> LmStatus,
    pub quadratic_add_entry: unsafe extern "C" fn(*mut LmQuadratic, u32, u32, f64) -> LmStatus,
    pub quadratic_add_quadratic:
        unsafe extern "C" fn(*mut LmQuadratic, *const LmQuadratic) -> LmStatus,
    pub quadratic_mul_bias: unsafe extern "C" fn(*mut LmQuadratic, f64) -> LmStatus,
    pub quadratic_clean: unsafe extern "C" fn(*mut LmQuadratic) -> LmStatus,
    pub quadratic_push_back_empty: unsafe extern "C" fn(*mut LmQuadratic, u32) -> LmStatus,
    pub quadratic_insert_empty: unsafe extern "C" fn(*mut LmQuadratic, usize, u32) -> LmStatus,
    pub quadratic_set_bias: unsafe extern "C" fn(*mut LmQuadratic, u32, u32, f64) -> LmStatus,
    pub quadratic_neg: unsafe extern "C" fn(*const LmQuadratic, *mut *mut LmQuadratic) -> LmStatus,
    // /////////// //
    // HigherOrder //
    // /////////// //
    pub higher_order_free: unsafe extern "C" fn(*mut LmHigherOrder),
    pub higher_order_default: extern "C" fn() -> *mut LmHigherOrder,
    pub higher_order_with_capacity: extern "C" fn(usize) -> *mut LmHigherOrder,
    pub higher_order_clone:
        unsafe extern "C" fn(*const LmHigherOrder, *mut *mut LmHigherOrder) -> LmStatus,
    pub higher_order_eq:
        unsafe extern "C" fn(*const LmHigherOrder, *const LmHigherOrder, *mut bool) -> LmStatus,
    pub higher_order_len: unsafe extern "C" fn(*const LmHigherOrder, *mut usize) -> LmStatus,
    pub higher_order_is_zero: unsafe extern "C" fn(*const LmHigherOrder, *mut bool) -> LmStatus,
    pub higher_order_is_empty: unsafe extern "C" fn(*const LmHigherOrder, *mut bool) -> LmStatus,
    pub higher_order_items:
        unsafe extern "C" fn(*const LmHigherOrder, *mut LmHigherOrderItemBuffer) -> LmStatus,
    pub higher_order_keys: unsafe extern "C" fn(*const LmHigherOrder, *mut LmStrBuffer) -> LmStatus,
    pub higher_order_degree: unsafe extern "C" fn(*const LmHigherOrder, *mut usize) -> LmStatus,
    pub higher_order_index:
        unsafe extern "C" fn(*const LmHigherOrder, *const u32, usize, *mut f64) -> LmStatus,
    pub higher_order_index_key:
        unsafe extern "C" fn(*const LmHigherOrder, *const c_char, *mut f64) -> LmStatus,
    pub higher_order_add_entry:
        unsafe extern "C" fn(*mut LmHigherOrder, *const u32, usize, f64) -> LmStatus,
    pub higher_order_add_higher_order:
        unsafe extern "C" fn(*mut LmHigherOrder, *const LmHigherOrder) -> LmStatus,
    pub higher_order_mul_bias: unsafe extern "C" fn(*mut LmHigherOrder, f64) -> LmStatus,
    pub higher_order_clean: unsafe extern "C" fn(*mut LmHigherOrder) -> LmStatus,
    pub higher_order_set_bias:
        unsafe extern "C" fn(*mut LmHigherOrder, *const u32, usize, f64) -> LmStatus,
    pub higher_order_set_bias_key:
        unsafe extern "C" fn(*mut LmHigherOrder, *const c_char, f64) -> LmStatus,
    pub higher_order_neg:
        unsafe extern "C" fn(*const LmHigherOrder, *mut *mut LmHigherOrder) -> LmStatus,
    // ///// //
    // Timer //
    // ///// //
    pub timer_free: unsafe extern "C" fn(*mut LmTimer),
    pub timer_start: extern "C" fn() -> *mut LmTimer,
    pub timer_record:
        unsafe extern "C" fn(*const LmTimer, *const c_char, *mut *mut LmSubTimer) -> LmStatus,
    pub timer_stop: unsafe extern "C" fn(*const LmTimer, *mut *mut LmTiming) -> LmStatus,
    // ////////// //
    // Sub-timer  //
    // ////////// //
    pub subtimer_free: unsafe extern "C" fn(*mut LmSubTimer),
    pub subtimer_stop: unsafe extern "C" fn(*mut LmSubTimer, *mut f64) -> LmStatus,
    // ////// //
    // Timing //
    // ////// //
    pub timing_free: unsafe extern "C" fn(*mut LmTiming),
    pub timing_new: unsafe extern "C" fn(f64, *mut *mut LmTiming) -> LmStatus,
    pub timing_clone: unsafe extern "C" fn(*const LmTiming, *mut *mut LmTiming) -> LmStatus,
    pub timing_eq: unsafe extern "C" fn(*const LmTiming, *const LmTiming, *mut bool) -> LmStatus,
    pub timing_total_seconds: unsafe extern "C" fn(*const LmTiming, *mut f64) -> LmStatus,
    pub timing_start_ns: unsafe extern "C" fn(*const LmTiming, *mut u64) -> LmStatus,
    pub timing_end_ns: unsafe extern "C" fn(*const LmTiming, *mut u64) -> LmStatus,
    pub timing_get:
        unsafe extern "C" fn(*const LmTiming, *const c_char, *mut LmF64Buffer) -> LmStatus,
    pub timing_total_for:
        unsafe extern "C" fn(*const LmTiming, *const c_char, *mut f64) -> LmStatus,
    pub timing_set: unsafe extern "C" fn(*mut LmTiming, *const c_char, f64) -> LmStatus,
    pub timing_add: unsafe extern "C" fn(*mut LmTiming, *const c_char, f64) -> LmStatus,
    pub timing_serialize: unsafe extern "C" fn(*const LmTiming, *mut LmBuffer) -> LmStatus,
    pub timing_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmTiming) -> LmStatus,
    pub timing_merge: unsafe extern "C" fn(*mut LmTiming, *const LmTiming) -> LmStatus,
    // ///// //
    // Error //
    // ///// //
    /// Frees an error handle. Passing null is a no-op.
    pub error_free: unsafe extern "C" fn(*mut LmError),
    /// Writes the error's kind tag (an open, stable string such as `"variable_not_existing"` or
    /// a plugin-defined `"myplugin::infeasible_subproblem"`) into `out`.
    pub error_kind: unsafe extern "C" fn(*const LmError, *mut LmBuffer) -> LmStatus,
    /// Writes the error's human-readable message into `out`.
    pub error_message: unsafe extern "C" fn(*const LmError, *mut LmBuffer) -> LmStatus,
    /// Returns the error's cause as a new owned handle in `out`, or `LmStatus::Absent` if this
    /// error has no cause.
    pub error_cause: unsafe extern "C" fn(*const LmError, *mut *mut LmError) -> LmStatus,
    /// Returns the current thread's last structured error, or null if none is set. Every
    /// fallible C API call clears this before running, so it always reflects the most recent
    /// call on this thread.
    pub last_error: extern "C" fn() -> *mut LmError,
    /// Sets the current thread's last structured error. Called by plugin/extension SDKs to
    /// report a pass failure with full fidelity (kind, message, cause chain) instead of just a
    /// non-`Ok` status code; the host folds it into the same error chain a native pass failure
    /// would produce. `kind` and `message` are copied immediately; `cause` (if non-null) is
    /// consumed.
    pub set_last_error:
        unsafe extern "C" fn(*const c_char, *const c_char, *mut LmError) -> LmStatus,
    // ///////////// //
    // Plugin passes //
    // ///////////// //
    /// Register a reversal handler for a plugin pass, keyed by its stable id.
    /// Call this (e.g. from `luna_model.register(...)`) before reversing a record that
    /// used the pass. `user_data` is owned by the plugin and must outlive any reversal
    /// (typically module-lifetime).
    pub register_plugin_backward:
        unsafe extern "C" fn(LmPassKind, *const c_char, LmBackwardFn, *mut c_void) -> LmStatus,
    /// Reads a plugin analysis result deposited under `name` by an analysis pass earlier in the
    /// same run, handing back its serialized bytes via `sink` so a plugin pass can consume another
    /// pass's analysis across the C-ABI (including one plugin consuming another plugin's analysis).
    ///
    /// `expected_type_tag` is the stable type tag the caller intends to decode into; the host
    /// verifies it against the tag recorded when the result was deposited, so a mismatched decode
    /// is rejected up front rather than producing garbage. Returns `Ok` and invokes `sink` exactly
    /// once with the result bytes when a result is present and the tags match; `Absent` when no
    /// result is stored under `name`; `InvalidArgument` when a result is stored but was produced by
    /// a different type than `expected_type_tag`. `sink` is not called on any non-`Ok` return.
    pub pass_context_get_analysis: unsafe extern "C" fn(
        ctx: *const LmPassContext,
        name: *const c_char,
        expected_type_tag: *const c_char,
        sink: LmByteSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Number of steps in the read-only view lent to a meta-analysis `run`. Writes the count to
    /// `out_len`.
    pub step_list_len:
        unsafe extern "C" fn(steps: *const LmStepList, out_len: *mut usize) -> LmStatus,
    /// Kind of the step at `index` (writes to `out_kind`). `OutOfBounds` if `index >= len`.
    pub step_list_kind: unsafe extern "C" fn(
        steps: *const LmStepList,
        index: usize,
        out_kind: *mut LmPassKind,
    ) -> LmStatus,
    /// Emits the name of the step at `index` via `sink` (once). `OutOfBounds` if `index >= len`.
    pub step_list_name: unsafe extern "C" fn(
        steps: *const LmStepList,
        index: usize,
        sink: LmStrSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Emits the analysis names the step at `index` requires, one per `sink` call.
    pub step_list_requires: unsafe extern "C" fn(
        steps: *const LmStepList,
        index: usize,
        sink: LmStrSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Emits the analysis names the step at `index` provides, one per `sink` call.
    pub step_list_provides: unsafe extern "C" fn(
        steps: *const LmStepList,
        index: usize,
        sink: LmStrSink,
        sink_ctx: *mut c_void,
    ) -> LmStatus,
    /// Writes the model parts the step at `index` may modify to `out_parts`. `OutOfBounds` if
    /// `index >= len`.
    pub step_list_modifies: unsafe extern "C" fn(
        steps: *const LmStepList,
        index: usize,
        out_parts: *mut LmModelParts,
    ) -> LmStatus,
    /// Creates a new control-flow execution plan for `name`, written to `out_plan`.
    pub control_flow_plan_new: unsafe extern "C" fn(
        name: *const c_char,
        out_plan: *mut *mut LmControlFlowPlan,
    ) -> LmStatus,
    /// Frees an unconsumed control-flow plan handle.
    pub control_flow_plan_free: unsafe extern "C" fn(plan: *mut LmControlFlowPlan),
    /// Appends a plugin transformation pass step to a control-flow plan.
    pub control_flow_plan_add_transform: unsafe extern "C" fn(
        plan: *mut LmControlFlowPlan,
        desc: *const LmPluginTransform,
    ) -> LmStatus,
    /// Appends a plugin analysis pass step to a control-flow plan.
    pub control_flow_plan_add_analysis: unsafe extern "C" fn(
        plan: *mut LmControlFlowPlan,
        desc: *const LmPluginAnalysis,
    ) -> LmStatus,
    /// Appends a plugin composite pass step to a control-flow plan.
    pub control_flow_plan_add_composite: unsafe extern "C" fn(
        plan: *mut LmControlFlowPlan,
        desc: *const LmPluginComposite,
    ) -> LmStatus,
    /// Appends a plugin control-flow pass step to a control-flow plan.
    pub control_flow_plan_add_control_flow: unsafe extern "C" fn(
        plan: *mut LmControlFlowPlan,
        desc: *const LmPluginControlFlow,
    ) -> LmStatus,
    /// Appends a plugin meta-analysis pass step to a control-flow plan.
    pub control_flow_plan_add_meta_analysis: unsafe extern "C" fn(
        plan: *mut LmControlFlowPlan,
        desc: *const LmPluginMetaAnalysis,
    ) -> LmStatus,
}

/// Size of the table as released in luna-model 0.8.0a1, the first release of ABI
/// version 1: the size of a host table whose [`LunaModelCApi::struct_size`] is `0`.
/// `control_flow_plan_add_meta_analysis` is that table's last entry point; this
/// stays fixed while the table grows.
///
/// cbindgen:ignore
pub const CAPI_BASE_TABLE_SIZE: usize =
    offset_of!(LunaModelCApi, control_flow_plan_add_meta_analysis) + size_of::<extern "C" fn()>();

// Layout pins. `struct_size` must stay in the padding after `abi_version` (tables of
// luna-model 0.8.0a1 have no such field) and the entry points must start at offset 8.
// Only 64-bit targets are built; there a function pointer is 8 bytes.
const _: () = {
    assert!(offset_of!(LunaModelCApi, abi_version) == 0);
    assert!(offset_of!(LunaModelCApi, struct_size) == 4);
    #[cfg(target_pointer_width = "64")]
    assert!(offset_of!(LunaModelCApi, version) == 8);
    assert!(size_of::<LunaModelCApi>() >= CAPI_BASE_TABLE_SIZE);
};

impl LunaModelCApi {
    /// Size in bytes of this (host) table: [`Self::struct_size`], or
    /// [`CAPI_BASE_TABLE_SIZE`] for a luna-model 0.8.0a1 table without that field.
    pub fn size(&self) -> usize {
        match self.struct_size {
            0 => CAPI_BASE_TABLE_SIZE,
            size => size as usize,
        }
    }
}

/// Whether the host table `$table` (`&LunaModelCApi`) contains entry point `$field`,
/// i.e. whether the host is recent enough to provide it.
///
/// Entry points of the base table ([`CAPI_BASE_TABLE_SIZE`]) are always present; check
/// every entry point appended later before calling it, since reading past the end of an
/// older host's table is undefined behaviour.
#[macro_export]
macro_rules! capi_has {
    ($table:expr, $field:ident) => {
        ::core::mem::offset_of!($crate::LunaModelCApi, $field)
            + ::core::mem::size_of::<unsafe extern "C" fn()>()
            <= $crate::LunaModelCApi::size($table)
    };
}
