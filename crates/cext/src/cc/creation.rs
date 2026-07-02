use crate::utils::{drop_from_c, handle_to_c};
use luna_model::core::ConstraintCollection;

use crate::LmCC;

/// Frees a constraint-collection handle. Passing null is a no-op.
///
/// # Safety
/// `cc` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_free(cc: *mut LmCC) {
    unsafe { drop_from_c(cc as *mut ConstraintCollection) }
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_default() -> *mut LmCC {
    handle_to_c(ConstraintCollection::default())
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_new() -> *mut LmCC {
    handle_to_c(ConstraintCollection::new())
}
