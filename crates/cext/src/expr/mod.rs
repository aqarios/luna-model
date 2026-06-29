mod access;
mod creation;
mod general;

pub use creation::{lm_expr_default, lm_expr_free, lm_expr_new, lm_expr_new_constant};
pub use general::lm_expr_clone;

pub use access::{
    lm_expr_complete_vars, lm_expr_degree, lm_expr_env, lm_expr_has_higher_order,
    lm_expr_has_quadratic, lm_expr_higher_order, lm_expr_is_constant, lm_expr_linear,
    lm_expr_num_vars, lm_expr_offset, lm_expr_quadratic, lm_expr_vars, lm_expr_vtypes,
};
