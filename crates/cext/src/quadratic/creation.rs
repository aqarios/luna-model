use crate::utils::{drop_from_c, handle_to_c};
use luna_model::core::prelude::Quadratic;

use crate::LmQuadratic;

/// Frees a quadratic handle. Passing null is a no-op.
///
/// # Safety
/// `quadratic` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_free(quadratic: *mut LmQuadratic) {
    unsafe { drop_from_c(quadratic as *mut Quadratic) }
}

/// Creates a new empty quadratic term.
///
/// Returns a non-null handle that must be released with [`lm_quadratic_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_quadratic_default() -> *mut LmQuadratic {
    handle_to_c(Quadratic::default())
}
