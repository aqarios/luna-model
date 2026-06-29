use luna_model::core::ConstraintCollection;

use crate::LmCC;

/// Frees a constraint-collection handle. Passing null is a no-op.
///
/// # Safety
/// `cc` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_cc_free(cc: *mut LmCC) {
    if !cc.is_null() {
        drop(unsafe { Box::from_raw(cc as *mut ConstraintCollection) });
    }
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_default() -> *mut LmCC {
    Box::into_raw(Box::new(ConstraintCollection::default())) as *mut LmCC
}

/// Creates a new empty constraint-collection.
///
/// Returns a non-null handle that must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_cc_new() -> *mut LmCC {
    Box::into_raw(Box::new(ConstraintCollection::new())) as *mut LmCC
}
