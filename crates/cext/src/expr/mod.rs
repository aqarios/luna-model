pub(crate) mod utils;

mod access;
mod creation;
mod equality;
mod general;
mod modification;
mod ops;
mod serialize;

pub use access::{
    lm_expr_complete_vars, lm_expr_degree, lm_expr_env, lm_expr_has_higher_order,
    lm_expr_has_quadratic, lm_expr_higher_order, lm_expr_is_constant, lm_expr_linear,
    lm_expr_num_vars, lm_expr_offset, lm_expr_quadratic, lm_expr_vars, lm_expr_vtypes,
};
pub use creation::{
    lm_expr_default, lm_expr_free, lm_expr_from_dense_quadratic, lm_expr_from_double,
    lm_expr_from_higher_order, lm_expr_from_linear, lm_expr_from_quadratic, lm_expr_from_size_t,
    lm_expr_new, lm_expr_new_constant,
};
pub use equality::{lm_expr_eq, lm_expr_equal_contents};
pub use general::{
    lm_expr_clone, lm_expr_deep_clone, lm_expr_deep_clone_many, lm_expr_evaluate_sample_quick,
};
pub use modification::{
    lm_expr_add_higher_order_term, lm_expr_add_linear_term, lm_expr_add_quadratic_term,
    lm_expr_set_offset,
};
pub use serialize::{lm_expr_deserialize, lm_expr_serialize};

pub use ops::{
    lm_expr_add_bias, lm_expr_add_expr, lm_expr_add_vref, lm_expr_iadd_bias, lm_expr_iadd_expr,
    lm_expr_iadd_vref, lm_expr_imul_bias, lm_expr_imul_expr, lm_expr_imul_vref, lm_expr_ipow,
    lm_expr_isub_bias, lm_expr_isub_expr, lm_expr_isub_vref, lm_expr_mul_bias, lm_expr_mul_expr,
    lm_expr_mul_vref, lm_expr_neg, lm_expr_pow, lm_expr_rsub_bias, lm_expr_separate,
    lm_expr_sub_bias, lm_expr_sub_expr, lm_expr_sub_vref, lm_expr_substitute,
};
