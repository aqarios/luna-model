mod access;

use luna_model::core::prelude::Variable;

use crate::LmVar;

/// Frees a variable handle. Passing null is a no-op.
///
/// # Safety
/// `var` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_var_free(var: *mut LmVar) {
    if !var.is_null() {
        drop(unsafe { Box::from_raw(var as *mut Variable) });
    }
}

pub use access::{lm_var_bounds, lm_var_inverted, lm_var_name, lm_var_vtype};
