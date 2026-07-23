use std::time::{Duration, UNIX_EPOCH};

use lunamodel_core::prelude::Timing;

use crate::{
    LmStatus, LmTiming,
    error::{CapiError, guard},
    utils::ref_from_c,
    utils::{drop_from_c, handle_to_c, opt_from_c, out_from_c},
};

/// Frees a timing handle. Passing null is a no-op.
///
/// # Safety
/// `timing` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_free(timing: *mut LmTiming) {
    unsafe { drop_from_c(timing as *mut Timing) }
}

fn unix_nanos(time: std::time::SystemTime) -> Result<u64, CapiError> {
    let nanos = time
        .duration_since(UNIX_EPOCH)
        .map_err(CapiError::errored)?
        .as_nanos();
    u64::try_from(nanos).map_err(CapiError::errored)
}

/// Returns the total duration in nanoseconds.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_total_ns(timing: *const LmTiming, out: *mut u64) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = u64::try_from(timing.total().map_err(CapiError::errored)?.as_nanos())
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Returns the start timestamp in Unix nanoseconds.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_start_ns(timing: *const LmTiming, out: *mut u64) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = unix_nanos(timing.start())?;
        Ok(())
    })
}

/// Returns the end timestamp in Unix nanoseconds.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_end_ns(timing: *const LmTiming, out: *mut u64) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = unix_nanos(timing.end())?;
        Ok(())
    })
}

/// Returns the optional QPU duration.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_qpu(timing: *const LmTiming, out: *mut f64) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = timing
            .qpu
            .ok_or_else(|| CapiError::new(LmStatus::Absent, "timing has no QPU duration"))?;
        Ok(())
    })
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

/// Clones a timing record.
///
/// # Safety
/// `timing` must be valid and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_clone(
    timing: *const LmTiming,
    out: *mut *mut LmTiming,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        *unsafe { out_from_c(out) }? = handle_to_c(*timing);
        Ok(())
    })
}

/// Compares two timing records.
///
/// # Safety
/// `timing` and `other` must be valid and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_eq(
    timing: *const LmTiming,
    other: *const LmTiming,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let other = unsafe { ref_from_c(other as *const Timing, "other") }?;
        *unsafe { out_from_c(out) }? = timing == other;
        Ok(())
    })
}
