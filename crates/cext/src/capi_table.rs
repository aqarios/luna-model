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
    vtype_buffer_free: crate::buffer::lm_vtype_buffer_free,
    vref_buffer_free: crate::buffer::lm_vref_buffer_free,
    cmp_buffer_free: crate::buffer::lm_cmp_buffer_free,
    ctype_buffer_free: crate::buffer::lm_ctype_buffer_free,
    u32_buffer_free: crate::buffer::lm_u32_buffer_free,
    linear_item_buffer_free: crate::buffer::lm_linear_item_buffer_free,
    quadratic_item_buffer_free: crate::buffer::lm_quadratic_item_buffer_free,
    higher_order_item_buffer_free: crate::buffer::lm_higher_order_item_buffer_free,
    // ///// //
    // MODEL //
    // ///// //
    model_free: crate::model::lm_model_free,
    // Model -- creation
    model_default: crate::model::lm_model_default,
    model_new: crate::model::lm_model_new,
    model_with_env: crate::model::lm_model_with_env,
    // Model -- general
    model_clone: crate::model::lm_model_clone,
    model_deep_clone: crate::model::lm_model_deep_clone,
    model_satisfies: crate::model::lm_model_satisfies,
    model_add_var: crate::model::lm_model_add_var,
    model_add_var_with_fallback: crate::model::lm_model_add_var_with_fallback,
    model_eq: crate::model::lm_model_eq,
    model_equal_contents: crate::model::lm_model_equal_contents,
    model_evaluate_solution: crate::model::lm_model_evaluate_solution,
    model_evaluate_solution_with_tol: crate::model::lm_model_evaluate_solution_with_tol,
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
    // Env -- general
    env_clone: crate::env::lm_env_clone,
    env_deep_clone: crate::env::lm_env_deep_clone,
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
    // Env -- modification
    env_insert: crate::env::lm_env_insert,
    env_insert_with_fallback: crate::env::lm_env_insert_with_fallback,
    env_insert_inverted: crate::env::lm_env_insert_inverted,
    env_remove: crate::env::lm_env_remove,
    // Env -- serializer
    env_serialize: crate::env::lm_env_serialize,
    env_deserialize: crate::env::lm_env_deserialize,
    // //////// //
    // SOLUTION //
    // //////// //
    sol_free: crate::solution::lm_sol_free,
    // Sol -- serializer
    sol_deserialize: crate::solution::lm_sol_deserialize,
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
    // Expr -- general
    expr_clone: crate::expr::lm_expr_clone,
    expr_deep_clone: crate::expr::lm_expr_deep_clone,
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
    expr_degree: crate::expr::lm_expr_degree,
    expr_is_constant: crate::expr::lm_expr_is_constant,
    expr_has_quadratic: crate::expr::lm_expr_has_quadratic,
    expr_has_higher_order: crate::expr::lm_expr_has_higher_order,
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
    // Constraint -- access
    constr_lhs: crate::constr::lm_constr_lhs,
    constr_rhs: crate::constr::lm_constr_rhs,
    constr_comparator: crate::constr::lm_constr_comparator,
    constr_name: crate::constr::lm_constr_name,
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
    // CC -- access
    cc_vtypes: crate::cc::lm_cc_vtypes,
    cc_ctypes: crate::cc::lm_cc_ctypes,
    cc_vars: crate::cc::lm_cc_vars,
    cc_complete_vars: crate::cc::lm_cc_complete_vars,
    cc_is_empty: crate::cc::lm_cc_is_empty,
    cc_len: crate::cc::lm_cc_len,
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
    // //////// //
    // Variable //
    // //////// //
    var_free: crate::var::lm_var_free,
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
    specs_satisfies: crate::specs::lm_specs_satisfies,
    specs_clone: crate::specs::lm_specs_clone,
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
    // Bounds -- access
    bounds_lower: crate::bounds::lm_bounds_lower,
    bounds_upper: crate::bounds::lm_bounds_upper,
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
    higher_order_degree: crate::higher_order::lm_higher_order_degree,
    higher_order_index: crate::higher_order::lm_higher_order_index,
    // HigherOrder -- modification
    higher_order_add_entry: crate::higher_order::lm_higher_order_add_entry,
    higher_order_add_higher_order: crate::higher_order::lm_higher_order_add_higher_order,
    higher_order_mul_bias: crate::higher_order::lm_higher_order_mul_bias,
    higher_order_set_bias: crate::higher_order::lm_higher_order_set_bias,
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
};
