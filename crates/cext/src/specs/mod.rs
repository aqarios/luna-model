mod access;
mod creation;
mod general;

pub use access::{
    lm_specs_ctypes, lm_specs_max_constraint_degree, lm_specs_max_degree,
    lm_specs_max_num_variables, lm_specs_sense, lm_specs_vtypes,
};
pub use creation::{lm_specs_free, lm_specs_new};
pub use general::{lm_specs_clone, lm_specs_satisfies};
