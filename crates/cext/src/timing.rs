use crate::utils::{drop_from_c, handle_to_c, opt_from_c, out_from_c};
use std::time::{Duration, UNIX_EPOCH};

use luna_model::core::prelude::Timing;

use crate::{LmStatus, LmTiming, error::guard};

/// Frees a timing handle. Passing null is a no-op.
///
/// # Safety
/// `timing` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_free(timing: *mut LmTiming) {
    unsafe { drop_from_c(timing as *mut Timing) }
}

/// Creates a new timing handle.
///
/// `start_unix_ns` and `end_unix_ns` are wall-clock timestamps in nanoseconds
/// since the UNIX epoch. `qpu` may be null to indicate no QPU time.
///
/// Returns a non-null handle in `out` that must be released with [`lm_timing_free`].
///
/// # Safety
/// `qpu` must be null or point to readable `double` storage.
/// `out` must point to writable `LmTiming *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_new(
    start_unix_ns: u64,
    end_unix_ns: u64,
    qpu: *const f64,
    out: *mut *mut LmTiming,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;

        let start = UNIX_EPOCH + Duration::from_nanos(start_unix_ns);
        let end = UNIX_EPOCH + Duration::from_nanos(end_unix_ns);
        let qpu = unsafe { opt_from_c(qpu) };
        *out = handle_to_c(Timing::new(start, end, qpu));
        Ok(())
    })
}
