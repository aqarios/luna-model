use luna_model::{core::prelude::Linear, types::Specs};

use crate::{
    LmLinear, LmStatus,
    error::{CapiError, guard},
};

/// Frees a linear handle. Passing null is a no-op.
///
/// # Safety
/// `linear` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_free(linear: *mut LmLinear) {
    if !linear.is_null() {
        drop(unsafe { Box::from_raw(linear as *mut Specs) });
    }
}

/// Creates a new empty linear term.
///
/// Returns a non-null handle that must be released with [`lm_linear_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_linear_default() -> *mut LmLinear {
    Box::into_raw(Box::new(Linear::default())) as *mut LmLinear
}

/// Creates a new linear term for a variable index and bias.
///
/// `out` receives an owned linear handle that must be freed with [`lm_linear_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_for_var(
    var: u32,
    bias: f64,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(Linear::for_var(var, bias))) as *mut LmLinear;
        Ok(())
    })
}
