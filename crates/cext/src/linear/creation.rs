use crate::utils::{drop_from_c, handle_to_c, out_from_c};
use luna_model::core::prelude::Linear;

use crate::{LmLinear, LmStatus, error::guard};

/// Frees a linear handle. Passing null is a no-op.
///
/// # Safety
/// `linear` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_free(linear: *mut LmLinear) {
    unsafe { drop_from_c(linear as *mut Linear) }
}

/// Creates a new empty linear term.
///
/// Returns a non-null handle that must be released with [`lm_linear_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_linear_default() -> *mut LmLinear {
    handle_to_c(Linear::default())
}

/// Creates a new linear term for a variable index and bias.
///
/// `out` receives an owned linear handle that must be freed with [`lm_linear_free`].
///
/// # Safety
/// `out` must point to writable `LmLinear *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_for_var(
    var: u32,
    bias: f64,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Linear::for_var(var, bias));
        Ok(())
    })
}

/// Creates a linear term containing two variable/bias entries.
///
/// # Safety
/// `out` must point to writable `LmLinear *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_for_vars(
    var_a: u32,
    bias_a: f64,
    var_b: u32,
    bias_b: f64,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Linear::for_vars((var_a, bias_a), (var_b, bias_b)));
        Ok(())
    })
}
