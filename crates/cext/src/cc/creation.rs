use luna_model::core::ConstraintCollection;

use super::utils::{collection_drop_from_c, collection_to_c};
use crate::LmCC;

/// Frees a constraint-collection handle. Passing null is a no-op.
///
/// # Safety
/// `cc` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_free(cc: *mut LmCC) {
    unsafe { collection_drop_from_c(cc) }
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_default() -> *mut LmCC {
    collection_to_c(ConstraintCollection::default())
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_new() -> *mut LmCC {
    collection_to_c(ConstraintCollection::new())
}
