mod access;
mod creation;
mod general;

pub use access::{
    lm_vref_bounds, lm_vref_check_living, lm_vref_hash, lm_vref_id, lm_vref_inverted, lm_vref_name,
    lm_vref_vtype,
};
pub use creation::{lm_vref_free, lm_vref_new};
pub use general::{lm_vref_eq, lm_vref_to_var};
