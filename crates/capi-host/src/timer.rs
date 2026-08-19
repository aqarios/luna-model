use std::ffi::c_char;

use lunamodel_core::SubTimer;
use lunamodel_core::prelude::Timer;

use crate::{
    LmStatus, LmSubTimer, LmTimer, LmTiming,
    error::{CapiError, guard},
    utils::{drop_from_c, handle_to_c, out_from_c, ref_from_c, str_from_c, take_from_c},
};

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
    handle_to_c(Timer::new())
}

/// Starts a fine-grained sub-timer under `key`, to be stopped and recorded
/// via [`lm_subtimer_stop`].
///
/// Returns a non-null handle that must eventually be released, either by
/// [`lm_subtimer_stop`] or [`lm_subtimer_free`] (which discards it unrecorded).
///
/// # Safety
/// `timer` must be a valid timer handle created by this library.
/// `key` must be a NUL-terminated UTF-8 string.
/// `out` must point to writable `LmSubTimer *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timer_record(
    timer: *const LmTimer,
    key: *const c_char,
    out: *mut *mut LmSubTimer,
) -> LmStatus {
    guard(|| {
        let timer = unsafe { ref_from_c(timer as *const Timer, "timer") }?;
        let key = unsafe { str_from_c(key, "key") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(timer.start(key));
        Ok(())
    })
}

/// Stops a timer, producing its accumulated [`LmTiming`].
///
/// The timer handle remains valid and reusable after this call.
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
        *out = handle_to_c(timer.clone().stop().map_err(CapiError::errored)?);
        Ok(())
    })
}

/// Frees a sub-timer handle without recording its elapsed time.
///
/// Passing null is a no-op.
///
/// # Safety
/// `subtimer` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_subtimer_free(subtimer: *mut LmSubTimer) {
    unsafe { drop_from_c(subtimer as *mut SubTimer) }
}

/// Stops a sub-timer, recording its elapsed time (in seconds) on the parent
/// timer under the key it was started with, and returns that elapsed time.
///
/// Consumes and frees `subtimer`; it must not be used or freed again.
///
/// # Safety
/// `subtimer` must be a valid sub-timer handle created by this library.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_subtimer_stop(subtimer: *mut LmSubTimer, out: *mut f64) -> LmStatus {
    guard(|| {
        let subtimer = unsafe { take_from_c(subtimer as *mut SubTimer, "subtimer") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = subtimer.stop().map_err(CapiError::errored)?;
        Ok(())
    })
}
