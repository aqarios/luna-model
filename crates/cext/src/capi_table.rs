//! The C API function-pointer table.
//!
//! [`LUNA_MODEL_C_API`] fills the `LunaModelCApi` table (defined in
//! `lunamodel-cext-vtable`) with this crate's `extern "C"` entry points. The
//! standalone library calls the `lm_*` symbols directly; the Python wheel publishes
//! a pointer to this table through a `PyCapsule` (`luna_model._lm._C_API`).

use crate::{CAPI_ABI_VERSION, LunaModelCApi};

/// The singleton C API table exposed to consumers (directly or via the capsule).
pub static LUNA_MODEL_C_API: LunaModelCApi = LunaModelCApi {
    abi_version: CAPI_ABI_VERSION,
    version: crate::version::lm_version,
    last_error_message: crate::error::lm_last_error_message,
    // ////// //
    // BUFFER //
    // ////// //
    buffer_free: crate::buffer::lm_buffer_free,
    str_buffer_free: crate::buffer::lm_str_buffer_free,
    vtype_buffer_free: crate::buffer::lm_vtype_buffer_free,
    ctype_buffer_free: crate::buffer::lm_ctype_buffer_free,
    cmp_buffer_free: crate::buffer::lm_cmp_buffer_free,
    vref_buffer_free: crate::buffer::lm_vref_buffer_free,
    column_buffer_free: crate::buffer::lm_column_buffer_free,
    u32_buffer_free: crate::buffer::lm_u32_buffer_free,
    f64_buffer_free: crate::buffer::lm_f64_buffer_free,
    linear_item_buffer_free: crate::buffer::lm_linear_item_buffer_free,
    quadratic_item_buffer_free: crate::buffer::lm_quadratic_item_buffer_free,
    higher_order_item_buffer_free: crate::buffer::lm_higher_order_item_buffer_free,
    result_view_buffer_free: crate::buffer::lm_result_view_buffer_free,
    sample_view_buffer_free: crate::buffer::lm_sample_view_buffer_free,
    index_map_buffer_free: crate::buffer::lm_index_map_buffer_free,
    bool_map_buffer_free: crate::buffer::lm_bool_map_buffer_free,
    // ///// //
    // MODEL //
    // ///// //
    model_free: crate::model::lm_model_free,
    // Model -- creation
    model_default: crate::model::lm_model_default,
    model_new: crate::model::lm_model_new,
    model_with_env: crate::model::lm_model_with_env,
    // Model -- general
    model_retain: crate::model::lm_model_retain,
    model_clone: crate::model::lm_model_clone,
    model_deep_clone: crate::model::lm_model_deep_clone,
    model_satisfies: crate::model::lm_model_satisfies,
    model_add_var: crate::model::lm_model_add_var,
    model_add_var_with_fallback: crate::model::lm_model_add_var_with_fallback,
    model_eq: crate::model::lm_model_eq,
    model_equal_contents: crate::model::lm_model_equal_contents,
    model_evaluate_solution: crate::model::lm_model_evaluate_solution,
    model_evaluate_solution_with_tol: crate::model::lm_model_evaluate_solution_with_tol,
    model_evaluate_sample: crate::model::lm_model_evaluate_sample,
    model_violated_constraints: crate::model::lm_model_violated_constraints,
    model_substitute: crate::model::lm_model_substitute,
    // Model -- access
    model_name: crate::model::lm_model_name,
    model_sense: crate::model::lm_model_sense,
    model_env: crate::model::lm_model_env,
    model_specs: crate::model::lm_model_specs,
    model_vtypes: crate::model::lm_model_vtypes,
    model_num_variables: crate::model::lm_model_num_variables,
    model_vars: crate::model::lm_model_vars,
    model_var: crate::model::lm_model_var,
    model_objective: crate::model::lm_model_objective,
    model_constraints: crate::model::lm_model_constraints,
    // Model -- setter
    model_set_name: crate::model::lm_model_set_name,
    model_set_sense: crate::model::lm_model_set_sense,
    model_set_objective: crate::model::lm_model_set_objective,
    model_set_constraints: crate::model::lm_model_set_constraints,
    // Model -- serializer
    model_serialize: crate::model::lm_model_serialize,
    model_deserialize: crate::model::lm_model_deserialize,
    // /////////// //
    // ENVIRONMENT //
    // /////////// //
    env_free: crate::env::lm_env_free,
    env_default: crate::env::lm_env_default,
    env_new: crate::env::lm_env_new,
    // Env -- general
    env_clone: crate::env::lm_env_clone,
    env_deep_clone: crate::env::lm_env_deep_clone,
    env_eq: crate::env::lm_env_eq,
    env_equal_contents: crate::env::lm_env_equal_contents,
    // Env -- access
    env_len: crate::env::lm_env_len,
    env_is_empty: crate::env::lm_env_is_empty,
    env_vars: crate::env::lm_env_vars,
    env_id: crate::env::lm_env_id,
    env_lookup: crate::env::lm_env_lookup,
    env_get: crate::env::lm_env_get,
    env_contains: crate::env::lm_env_contains,
    env_vtype_of: crate::env::lm_env_vtype_of,
    env_sort: crate::env::lm_env_sort,
    env_next_idx: crate::env::lm_env_next_idx,
    // Env -- modification
    env_insert: crate::env::lm_env_insert,
    env_insert_with_fallback: crate::env::lm_env_insert_with_fallback,
    env_insert_inverted: crate::env::lm_env_insert_inverted,
    env_remove: crate::env::lm_env_remove,
    env_set_inverted: crate::env::lm_env_set_inverted,
    // Env -- serializer
    env_serialize: crate::env::lm_env_serialize,
    env_deserialize: crate::env::lm_env_deserialize,
    // ////// //
    // COLUMN //
    // ////// //
    col_free: crate::col::lm_col_free,
    // Column -- creation
    col_new: crate::col::lm_col_new,
    col_with: crate::col::lm_col_with,
    // Column -- general
    col_clone: crate::col::lm_col_clone,
    col_eq: crate::col::lm_col_eq,
    col_filter_by_mask: crate::col::lm_col_filter_by_mask,
    col_extract: crate::col::lm_col_extract,
    // Column -- access
    col_len: crate::col::lm_col_len,
    col_is_empty: crate::col::lm_col_is_empty,
    col_vtype: crate::col::lm_col_vtype,
    col_assignment: crate::col::lm_col_assignment,
    col_values: crate::col::lm_col_values,
    // Column -- modification
    col_push: crate::col::lm_col_push,
    col_try_push: crate::col::lm_col_try_push,
    col_remove: crate::col::lm_col_remove,
    // /////////// //
    // RESULT_VIEW //
    // /////////// //
    result_view_free: crate::res::lm_result_view_free,
    result_view_counts: crate::res::lm_result_view_counts,
    result_view_obj_value: crate::res::lm_result_view_obj_value,
    result_view_raw_energy: crate::res::lm_result_view_raw_energy,
    result_view_constraints: crate::res::lm_result_view_constraints,
    result_view_variable_bounds: crate::res::lm_result_view_variable_bounds,
    result_view_feasible: crate::res::lm_result_view_feasible,
    result_view_sample: crate::res::lm_result_view_sample,
    // /////////// //
    // SAMPLE_VIEW //
    // /////////// //
    sample_view_free: crate::sample::lm_sample_view_free,
    sample_view_index: crate::sample::lm_sample_view_index,
    sample_view_variable_names: crate::sample::lm_sample_view_variable_names,
    sample_view_assignment: crate::sample::lm_sample_view_assignment,
    sample_view_assignment_idx: crate::sample::lm_sample_view_assignment_idx,
    // //////// //
    // SOLUTION //
    // //////// //
    sol_free: crate::sol::lm_sol_free,
    sol_default: crate::sol::lm_sol_default,
    sol_with_sense: crate::sol::lm_sol_with_sense,
    sol_with_capacity: crate::sol::lm_sol_with_capacity,
    sol_from_env: crate::sol::lm_sol_from_env,
    sol_from_random: crate::sol::lm_sol_from_random,
    // Sol -- general
    sol_clone: crate::sol::lm_sol_clone,
    sol_eq: crate::sol::lm_sol_eq,
    sol_equal_contents: crate::sol::lm_sol_equal_contents,
    sol_aggregate: crate::sol::lm_sol_aggregate,
    sol_merge_many: crate::sol::lm_sol_merge_many,
    sol_filter_by_mask: crate::sol::lm_sol_filter_by_mask,
    // Sol -- properties
    sol_samples: crate::sol::lm_sol_samples,
    sol_column: crate::sol::lm_sol_column,
    sol_counts: crate::sol::lm_sol_counts,
    sol_raw_energies: crate::sol::lm_sol_raw_energies,
    sol_obj_values: crate::sol::lm_sol_obj_values,
    sol_feasible: crate::sol::lm_sol_feasible,
    sol_constraints: crate::sol::lm_sol_constraints,
    sol_variable_bounds: crate::sol::lm_sol_variable_bounds,
    sol_timing: crate::sol::lm_sol_timing,
    sol_sense: crate::sol::lm_sol_sense,
    // Sol -- access
    sol_n_samples: crate::sol::lm_sol_n_samples,
    sol_len: crate::sol::lm_sol_len,
    sol_variable_names: crate::sol::lm_sol_variable_names,
    sol_is_empty: crate::sol::lm_sol_is_empty,
    sol_sample_len: crate::sol::lm_sol_sample_len,
    sol_value: crate::sol::lm_sol_value,
    sol_assignment: crate::sol::lm_sol_assignment,
    sol_assignment_idx: crate::sol::lm_sol_assignment_idx,
    sol_result: crate::sol::lm_sol_result,
    sol_results: crate::sol::lm_sol_results,
    sol_sample_view: crate::sol::lm_sol_sample_view,
    sol_sample_views: crate::sol::lm_sol_sample_views,
    sol_best: crate::sol::lm_sol_best,
    sol_extract: crate::sol::lm_sol_extract,
    // Sol -- modification
    sol_push: crate::sol::lm_sol_push,
    sol_add_empty_binary: crate::sol::lm_sol_add_empty_binary,
    sol_add_empty_spin: crate::sol::lm_sol_add_empty_spin,
    sol_add_empty_integer: crate::sol::lm_sol_add_empty_integer,
    sol_add_empty_real: crate::sol::lm_sol_add_empty_real,
    sol_add_binary: crate::sol::lm_sol_add_binary,
    sol_add_spin: crate::sol::lm_sol_add_spin,
    sol_add_integer: crate::sol::lm_sol_add_integer,
    sol_add_real: crate::sol::lm_sol_add_real,
    sol_add_col: crate::sol::lm_sol_add_col,
    sol_remove_col: crate::sol::lm_sol_remove_col,
    sol_remove_cols: crate::sol::lm_sol_remove_cols,
    sol_set_counts: crate::sol::lm_sol_set_counts,
    sol_set_timing: crate::sol::lm_sol_set_timing,
    sol_set_sense: crate::sol::lm_sol_set_sense,
    // Sol -- convenience
    sol_cvar: crate::sol::lm_sol_cvar,
    sol_expectation_value: crate::sol::lm_sol_expectation_value,
    sol_temperature_weighted: crate::sol::lm_sol_temperature_weighted,
    sol_feasibility_ratio: crate::sol::lm_sol_feasibility_ratio,
    sol_highest_constraint_violations: crate::sol::lm_sol_highest_constraint_violations,
    // Sol -- serializer
    sol_serialize: crate::sol::lm_sol_serialize,
    sol_deserialize: crate::sol::lm_sol_deserialize,
    // ////////// //
    // EXPRESSION //
    // ////////// //
    // Expr -- creation
    expr_free: crate::expr::lm_expr_free,
    expr_default: crate::expr::lm_expr_default,
    expr_new: crate::expr::lm_expr_new,
    expr_new_constant: crate::expr::lm_expr_new_constant,
    expr_from_double: crate::expr::lm_expr_from_double,
    expr_from_size_t: crate::expr::lm_expr_from_size_t,
    expr_from_linear: crate::expr::lm_expr_from_linear,
    expr_from_quadratic: crate::expr::lm_expr_from_quadratic,
    expr_from_higher_order: crate::expr::lm_expr_from_higher_order,
    expr_from_dense_quadratic: crate::expr::lm_expr_from_dense_quadratic,
    // Expr -- general
    expr_clone: crate::expr::lm_expr_clone,
    expr_deep_clone: crate::expr::lm_expr_deep_clone,
    expr_deep_clone_many: crate::expr::lm_expr_deep_clone_many,
    expr_evaluate_sample_quick: crate::expr::lm_expr_evaluate_sample_quick,
    expr_evaluate_sample: crate::expr::lm_expr_evaluate_sample,
    expr_evaluate: crate::expr::lm_expr_evaluate,
    // Expr -- serializer
    expr_serialize: crate::expr::lm_expr_serialize,
    expr_deserialize: crate::expr::lm_expr_deserialize,
    // Expr -- equality
    expr_eq: crate::expr::lm_expr_eq,
    expr_equal_contents: crate::expr::lm_expr_equal_contents,
    // Expr -- access
    expr_env: crate::expr::lm_expr_env,
    expr_offset: crate::expr::lm_expr_offset,
    expr_linear: crate::expr::lm_expr_linear,
    expr_quadratic: crate::expr::lm_expr_quadratic,
    expr_higher_order: crate::expr::lm_expr_higher_order,
    expr_num_vars: crate::expr::lm_expr_num_vars,
    expr_vtypes: crate::expr::lm_expr_vtypes,
    expr_vars: crate::expr::lm_expr_vars,
    expr_complete_vars: crate::expr::lm_expr_complete_vars,
    expr_contains: crate::expr::lm_expr_contains,
    expr_degree: crate::expr::lm_expr_degree,
    expr_is_constant: crate::expr::lm_expr_is_constant,
    expr_has_quadratic: crate::expr::lm_expr_has_quadratic,
    expr_has_higher_order: crate::expr::lm_expr_has_higher_order,
    // Expr -- modification
    expr_set_offset: crate::expr::lm_expr_set_offset,
    expr_add_linear_term: crate::expr::lm_expr_add_linear_term,
    expr_add_quadratic_term: crate::expr::lm_expr_add_quadratic_term,
    expr_add_higher_order_term: crate::expr::lm_expr_add_higher_order_term,
    // Expr -- ops -- add
    expr_add_bias: crate::expr::lm_expr_add_bias,
    expr_add_vref: crate::expr::lm_expr_add_vref,
    expr_add_expr: crate::expr::lm_expr_add_expr,
    // Expr -- ops -- sub
    expr_sub_bias: crate::expr::lm_expr_sub_bias,
    expr_rsub_bias: crate::expr::lm_expr_rsub_bias,
    expr_sub_vref: crate::expr::lm_expr_sub_vref,
    expr_sub_expr: crate::expr::lm_expr_sub_expr,
    // Expr -- ops -- mul
    expr_mul_bias: crate::expr::lm_expr_mul_bias,
    expr_mul_vref: crate::expr::lm_expr_mul_vref,
    expr_mul_expr: crate::expr::lm_expr_mul_expr,
    // Expr -- ops -- iadd
    expr_iadd_bias: crate::expr::lm_expr_iadd_bias,
    expr_iadd_vref: crate::expr::lm_expr_iadd_vref,
    expr_iadd_expr: crate::expr::lm_expr_iadd_expr,
    // Expr -- ops -- isub
    expr_isub_bias: crate::expr::lm_expr_isub_bias,
    expr_isub_vref: crate::expr::lm_expr_isub_vref,
    expr_isub_expr: crate::expr::lm_expr_isub_expr,
    // Expr -- ops -- imul
    expr_imul_bias: crate::expr::lm_expr_imul_bias,
    expr_imul_vref: crate::expr::lm_expr_imul_vref,
    expr_imul_expr: crate::expr::lm_expr_imul_expr,
    // Expr -- ops
    expr_neg: crate::expr::lm_expr_neg,
    expr_pow: crate::expr::lm_expr_pow,
    expr_ipow: crate::expr::lm_expr_ipow,
    expr_separate: crate::expr::lm_expr_separate,
    expr_substitute: crate::expr::lm_expr_substitute,
    // ////////// //
    // Constraint //
    // ////////// //
    constr_free: crate::constr::lm_constr_free,
    // Constraint -- creation
    constr_new: crate::constr::lm_constr_new,
    // Constraint -- general
    constr_clone: crate::constr::lm_constr_clone,
    constr_deep_clone: crate::constr::lm_constr_deep_clone,
    constr_substitute: crate::constr::lm_constr_substitute,
    constr_equal_contents: crate::constr::lm_constr_equal_contents,
    constr_eq: crate::constr::lm_constr_eq,
    constr_evaluate_sample: crate::constr::lm_constr_evaluate_sample,
    constr_evaluate_sample_quick: crate::constr::lm_constr_evaluate_sample_quick,
    // Constraint -- access
    constr_lhs: crate::constr::lm_constr_lhs,
    constr_rhs: crate::constr::lm_constr_rhs,
    constr_comparator: crate::constr::lm_constr_comparator,
    constr_name: crate::constr::lm_constr_name,
    constr_auto_name: crate::constr::lm_constr_auto_name,
    // Constraint -- set
    constr_set_name: crate::constr::lm_constr_set_name,
    // //////////////////// //
    // ConstraintCollection //
    // //////////////////// //
    cc_free: crate::cc::lm_cc_free,
    // CC -- creation
    cc_default: crate::cc::lm_cc_default,
    cc_new: crate::cc::lm_cc_new,
    // CC -- general
    cc_clone: crate::cc::lm_cc_clone,
    cc_deep_clone: crate::cc::lm_cc_deep_clone,
    cc_eq: crate::cc::lm_cc_eq,
    cc_equal_contents: crate::cc::lm_cc_equal_contents,
    cc_evaluate_sample: crate::cc::lm_cc_evaluate_sample,
    cc_evaluate_sample_quick: crate::cc::lm_cc_evaluate_sample_quick,
    // CC -- access
    cc_vtypes: crate::cc::lm_cc_vtypes,
    cc_ctypes: crate::cc::lm_cc_ctypes,
    cc_vars: crate::cc::lm_cc_vars,
    cc_complete_vars: crate::cc::lm_cc_complete_vars,
    cc_is_empty: crate::cc::lm_cc_is_empty,
    cc_len: crate::cc::lm_cc_len,
    cc_names: crate::cc::lm_cc_names,
    cc_contains: crate::cc::lm_cc_contains,
    cc_get: crate::cc::lm_cc_get,
    // CC -- modification
    cc_add_constraint: crate::cc::lm_cc_add_constraint,
    cc_add_collection: crate::cc::lm_cc_add_collection,
    cc_set_constraint: crate::cc::lm_cc_set_constraint,
    cc_remove_constraint: crate::cc::lm_cc_remove_constraint,
    cc_substitute: crate::cc::lm_cc_substitute,
    // CC -- serializer
    cc_serialize: crate::cc::lm_cc_serialize,
    cc_deserialize: crate::cc::lm_cc_deserialize,
    // ////// //
    // VarRef //
    // ////// //
    vref_free: crate::vref::lm_vref_free,
    // Vref -- creation
    vref_new: crate::vref::lm_vref_new,
    // Vref -- general
    vref_clone: crate::vref::lm_vref_clone,
    // Vref -- access
    vref_id: crate::vref::lm_vref_id,
    vref_check_living: crate::vref::lm_vref_check_living,
    vref_name: crate::vref::lm_vref_name,
    vref_vtype: crate::vref::lm_vref_vtype,
    vref_bounds: crate::vref::lm_vref_bounds,
    vref_inverted: crate::vref::lm_vref_inverted,
    vref_hash: crate::vref::lm_vref_hash,
    vref_eq: crate::vref::lm_vref_eq,
    vref_to_var: crate::vref::lm_vref_to_var,
    // Vref -- ops
    vref_mul_vref: crate::vref::lm_vref_mul_vref,
    vref_mul_bias: crate::vref::lm_vref_mul_bias,
    vref_add_vref: crate::vref::lm_vref_add_vref,
    vref_add_bias: crate::vref::lm_vref_add_bias,
    vref_sub_vref: crate::vref::lm_vref_sub_vref,
    vref_sub_expr: crate::vref::lm_vref_sub_expr,
    vref_sub_bias: crate::vref::lm_vref_sub_bias,
    vref_rsub_bias: crate::vref::lm_vref_rsub_bias,
    vref_neg: crate::vref::lm_vref_neg,
    vref_not: crate::vref::lm_vref_not,
    vref_pow: crate::vref::lm_vref_pow,
    vref_eval: crate::vref::lm_vref_eval,
    // //////// //
    // Variable //
    // //////// //
    var_free: crate::var::lm_var_free,
    // Variable -- creation
    var_new: crate::var::lm_var_new,
    // Variable -- general
    var_clone: crate::var::lm_var_clone,
    var_eq: crate::var::lm_var_eq,
    // Variable -- access
    var_name: crate::var::lm_var_name,
    var_vtype: crate::var::lm_var_vtype,
    var_bounds: crate::var::lm_var_bounds,
    var_inverted: crate::var::lm_var_inverted,
    // ///// //
    // Specs //
    // ///// //
    specs_free: crate::specs::lm_specs_free,
    // Specs -- creation
    specs_new: crate::specs::lm_specs_new,
    // Specs -- general
    specs_clone: crate::specs::lm_specs_clone,
    specs_satisfies: crate::specs::lm_specs_satisfies,
    specs_diff: crate::specs::lm_specs_diff,
    // Specs -- access
    specs_sense: crate::specs::lm_specs_sense,
    specs_vtypes: crate::specs::lm_specs_vtypes,
    specs_ctypes: crate::specs::lm_specs_ctypes,
    specs_max_degree: crate::specs::lm_specs_max_degree,
    specs_max_constraint_degree: crate::specs::lm_specs_max_constraint_degree,
    specs_max_num_variables: crate::specs::lm_specs_max_num_variables,
    // ///// //
    // Bound //
    // ///// //
    bound_is_bounded: crate::bound::lm_bound_is_bounded,
    bound_value: crate::bound::lm_bound_value,
    bound_eq: crate::bound::lm_bound_eq,
    // ////// //
    // Bounds //
    // ////// //
    bounds_free: crate::bounds::lm_bounds_free,
    // Bounds -- creation
    bounds_new: crate::bounds::lm_bounds_new,
    bounds_binary: crate::bounds::lm_bounds_binary,
    bounds_spin: crate::bounds::lm_bounds_spin,
    bounds_integer: crate::bounds::lm_bounds_integer,
    bounds_real: crate::bounds::lm_bounds_real,
    bounds_default_for: crate::bounds::lm_bounds_default_for,
    // Bounds -- general
    bounds_clone: crate::bounds::lm_bounds_clone,
    bounds_eq: crate::bounds::lm_bounds_eq,
    // Bounds -- access
    bounds_lower: crate::bounds::lm_bounds_lower,
    bounds_upper: crate::bounds::lm_bounds_upper,
    // Bounds -- ops
    bounds_eval: crate::bounds::lm_bounds_eval,
    // ////// //
    // LazyBounds //
    // ////// //
    lazy_bounds_free: crate::lazy_bounds::lm_lazy_bounds_free,
    // Bounds -- creation
    lazy_bounds_default: crate::lazy_bounds::lm_lazy_bounds_default,
    lazy_bounds_new: crate::lazy_bounds::lm_lazy_bounds_new,
    // Bounds -- general
    lazy_bounds_concretize: crate::lazy_bounds::lm_lazy_bounds_concretize,
    lazy_bounds_clone: crate::lazy_bounds::lm_lazy_bounds_clone,
    // Bounds -- access
    lazy_bounds_lower: crate::lazy_bounds::lm_lazy_bounds_lower,
    lazy_bounds_upper: crate::lazy_bounds::lm_lazy_bounds_upper,
    // ////// //
    // Linear //
    // ////// //
    linear_free: crate::linear::lm_linear_free,
    // Linear -- creation
    linear_default: crate::linear::lm_linear_default,
    linear_for_var: crate::linear::lm_linear_for_var,
    linear_for_vars: crate::linear::lm_linear_for_vars,
    // Linear -- general
    linear_clone: crate::linear::lm_linear_clone,
    linear_eq: crate::linear::lm_linear_eq,
    // Linear -- access
    linear_len: crate::linear::lm_linear_len,
    linear_is_zero: crate::linear::lm_linear_is_zero,
    linear_is_empty: crate::linear::lm_linear_is_empty,
    linear_items: crate::linear::lm_linear_items,
    linear_index: crate::linear::lm_linear_index,
    // Linear -- modificattion
    linear_add_entry: crate::linear::lm_linear_add_entry,
    linear_add_linear: crate::linear::lm_linear_add_linear,
    linear_mul_bias: crate::linear::lm_linear_mul_bias,
    linear_set_bias: crate::linear::lm_linear_set_bias,
    linear_neg: crate::linear::lm_linear_neg,
    // ///////// //
    // Quadratic //
    // ///////// //
    quadratic_free: crate::quadratic::lm_quadratic_free,
    // Quadratic -- creation
    quadratic_default: crate::quadratic::lm_quadratic_default,
    // Quadratic -- general
    quadratic_clone: crate::quadratic::lm_quadratic_clone,
    quadratic_eq: crate::quadratic::lm_quadratic_eq,
    // Quadratic -- access
    quadratic_len: crate::quadratic::lm_quadratic_len,
    quadratic_is_zero: crate::quadratic::lm_quadratic_is_zero,
    quadratic_is_empty: crate::quadratic::lm_quadratic_is_empty,
    quadratic_items: crate::quadratic::lm_quadratic_items,
    quadratic_index: crate::quadratic::lm_quadratic_index,
    // Quadratic -- modificattion
    quadratic_add_entry: crate::quadratic::lm_quadratic_add_entry,
    quadratic_add_quadratic: crate::quadratic::lm_quadratic_add_quadratic,
    quadratic_mul_bias: crate::quadratic::lm_quadratic_mul_bias,
    quadratic_clean: crate::quadratic::lm_quadratic_clean,
    quadratic_push_back_empty: crate::quadratic::lm_quadratic_push_back_empty,
    quadratic_insert_empty: crate::quadratic::lm_quadratic_insert_empty,
    quadratic_set_bias: crate::quadratic::lm_quadratic_set_bias,
    quadratic_neg: crate::quadratic::lm_quadratic_neg,
    // /////////// //
    // HigherOrder //
    // /////////// //
    higher_order_free: crate::higher_order::lm_higher_order_free,
    // HigherOrder -- creation
    higher_order_default: crate::higher_order::lm_higher_order_default,
    higher_order_with_capacity: crate::higher_order::lm_higher_order_with_capacity,
    // HigherOrder -- general
    higher_order_clone: crate::higher_order::lm_higher_order_clone,
    higher_order_eq: crate::higher_order::lm_higher_order_eq,
    // HigherOrder -- access
    higher_order_len: crate::higher_order::lm_higher_order_len,
    higher_order_is_zero: crate::higher_order::lm_higher_order_is_zero,
    higher_order_is_empty: crate::higher_order::lm_higher_order_is_empty,
    higher_order_items: crate::higher_order::lm_higher_order_items,
    higher_order_keys: crate::higher_order::lm_higher_order_keys,
    higher_order_degree: crate::higher_order::lm_higher_order_degree,
    higher_order_index: crate::higher_order::lm_higher_order_index,
    higher_order_index_key: crate::higher_order::lm_higher_order_index_key,
    // HigherOrder -- modification
    higher_order_add_entry: crate::higher_order::lm_higher_order_add_entry,
    higher_order_add_higher_order: crate::higher_order::lm_higher_order_add_higher_order,
    higher_order_mul_bias: crate::higher_order::lm_higher_order_mul_bias,
    higher_order_clean: crate::higher_order::lm_higher_order_clean,
    higher_order_set_bias: crate::higher_order::lm_higher_order_set_bias,
    higher_order_set_bias_key: crate::higher_order::lm_higher_order_set_bias_key,
    higher_order_neg: crate::higher_order::lm_higher_order_neg,
    // ///// //
    // Timer //
    // ///// //
    timer_free: crate::timer::lm_timer_free,
    timer_start: crate::timer::lm_timer_start,
    timer_stop: crate::timer::lm_timer_stop,
    // ////// //
    // Timing //
    // ////// //
    timing_free: crate::timing::lm_timing_free,
    timing_new: crate::timing::lm_timing_new,
    timing_clone: crate::timing::lm_timing_clone,
    timing_eq: crate::timing::lm_timing_eq,
    timing_total_ns: crate::timing::lm_timing_total_ns,
    timing_start_ns: crate::timing::lm_timing_start_ns,
    timing_end_ns: crate::timing::lm_timing_end_ns,
    timing_qpu: crate::timing::lm_timing_qpu,
};
