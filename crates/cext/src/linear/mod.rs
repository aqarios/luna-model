mod access;
mod creation;
mod general;
mod modification;

pub use access::{
    lm_linear_index, lm_linear_is_empty, lm_linear_is_zero, lm_linear_items, lm_linear_len,
};
pub use creation::{lm_linear_default, lm_linear_for_var, lm_linear_free};
pub use general::{lm_linear_clone, lm_linear_eq};
pub use modification::{
    lm_linear_add_entry, lm_linear_add_linear, lm_linear_mul_bias, lm_linear_neg,
    lm_linear_set_bias,
};
