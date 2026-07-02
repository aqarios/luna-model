use crate::utils::{drop_from_c, handle_to_c, out_from_c, ref_from_c};
use luna_model::core::prelude::Timer;

use crate::{LmStatus, LmTimer, LmTiming, error::guard};

/// Frees a timer handle. Passing null is a no-op.
///
/// # Safety
/// `timer` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timer_free(timer: *mut LmTimer) {
    unsafe { drop_from_c(timer as *mut Timer) }
}

/// Creates a new timer by starting it.
///
/// Returns a non-null handle that must be released with [`lm_timer_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_timer_start() -> *mut LmTimer {
    handle_to_c(Timer::start())
}

/// Stops a timer.
///
/// Returns a non-null handle that must be released with [`lm_timing_free`].
///
/// # Safety
/// `timer` must be a valid timer handle created by this library.
/// `out` must point to writable `LmTiming *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timer_stop(timer: *const LmTimer, out: *mut *mut LmTiming) -> LmStatus {
    guard(|| {
        let timer = unsafe { ref_from_c(timer as *const Timer, "timer") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(timer.stop());
        Ok(())
    })
}
