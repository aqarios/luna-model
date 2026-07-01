use luna_model::core::prelude::Timing;

use crate::LmTiming;

/// Frees a timing handle. Passing null is a no-op.
///
/// # Safety
/// `timing` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_free(timing: *mut LmTiming) {
    if !timing.is_null() {
        drop(unsafe { Box::from_raw(timing as *mut Timing) });
    }
}
