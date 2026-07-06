use std::ffi::c_char;

use luna_model::core::Solution;

use crate::{
    LmSol, LmStatus, LmVtype,
    error::{CapiError, guard},
    utils::{mut_from_c, opt_from_c, slice_from_c, str_from_c, vtype_from_c},
};
