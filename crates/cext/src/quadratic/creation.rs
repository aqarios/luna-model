use luna_model::{core::prelude::Quadratic, types::Specs};

use crate::LmQuadratic;

/// Frees a quadratic handle. Passing null is a no-op.
///
/// # Safety
/// `quadratic` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_free(quadratic: *mut LmQuadratic) {
    if !quadratic.is_null() {
        drop(unsafe { Box::from_raw(quadratic as *mut Specs) });
    }
}

/// Creates a new empty quadratic term.
///
/// Returns a non-null handle that must be released with [`lm_quadratic_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_quadratic_default() -> *mut LmQuadratic {
    Box::into_raw(Box::new(Quadratic::default())) as *mut LmQuadratic
}
