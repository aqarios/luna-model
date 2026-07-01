use luna_model::core::prelude::Timer;

use crate::{
    LmStatus, LmTimer, LmTiming,
    error::{CapiError, guard},
};

/// Frees a timer handle. Passing null is a no-op.
///
/// # Safety
/// `timer` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timer_free(timer: *mut LmTimer) {
    if !timer.is_null() {
        drop(unsafe { Box::from_raw(timer as *mut Timer) });
    }
}

/// Creates a new timer by starting it.
///
/// Returns a non-null handle that must be released with [`lm_timer_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_timer_start() -> *mut LmTimer {
    Box::into_raw(Box::new(Timer::start())) as *mut LmTimer
}

/// Stops a timer.
///
/// Returns a non-null handle that must be released with [`lm_timing_free`].
///
/// # Safety
/// `timer` must be a valid timer handle created by this library.
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub extern "C" fn lm_timer_stop(timer: *const LmTimer, out: *mut *mut LmTiming) -> LmStatus {
    guard(|| {
        let timer = unsafe { (timer as *const Timer).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("timer"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(timer.stop())) as *mut LmTiming;
        Ok(())
    })
}
