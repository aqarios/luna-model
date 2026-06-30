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
    // Model -- setter
    model_set_name: crate::model::lm_model_set_name,
    model_set_sense: crate::model::lm_model_set_sense,
    model_set_objective: crate::model::lm_model_set_objective,
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
    constr_new: crate::constr::lm_constr_new,
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
};
