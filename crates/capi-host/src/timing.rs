use std::ffi::c_char;
use std::time::UNIX_EPOCH;

use lunamodel_core::prelude::Timing;
use lunamodel_serializer::prelude::*;

use crate::{
    LmBuffer, LmF64Buffer, LmStatus, LmTiming,
    buffer::{buffer_from_vec, f64_buffer_from_vec},
    error::{CapiError, guard},
    utils::ref_from_c,
    utils::{drop_from_c, handle_to_c, mut_from_c, out_from_c, slice_from_c, str_from_c},
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

/// Returns the total duration in seconds.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_total_seconds(
    timing: *const LmTiming,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = timing.total;
        Ok(())
    })
}

/// Returns the start timestamp in Unix nanoseconds.
///
/// Returns `LmStatus::Absent` when the timing has no recorded start time.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_start_ns(timing: *const LmTiming, out: *mut u64) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        let start = timing
            .start
            .ok_or_else(|| CapiError::new(LmStatus::Absent, "timing has no start time"))?;
        *out = unix_nanos(start)?;
        Ok(())
    })
}

/// Returns the end timestamp in Unix nanoseconds.
///
/// Returns `LmStatus::Absent` when the timing has no recorded end time.
///
/// # Safety
/// `timing` must be a valid timing handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_end_ns(timing: *const LmTiming, out: *mut u64) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        let end = timing
            .end
            .ok_or_else(|| CapiError::new(LmStatus::Absent, "timing has no end time"))?;
        *out = unix_nanos(end)?;
        Ok(())
    })
}

/// Returns the values recorded under `key`, or an empty buffer if absent.
///
/// The buffer must be released with `lm_f64_buffer_free`.
///
/// # Safety
/// `timing` must be a valid timing handle. `key` must be a NUL-terminated
/// UTF-8 string. `out` must point to writable `LmF64Buffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_get(
    timing: *const LmTiming,
    key: *const c_char,
    out: *mut LmF64Buffer,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let key = unsafe { str_from_c(key, "key") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = f64_buffer_from_vec(timing.get(key).to_vec());
        Ok(())
    })
}

/// Returns the sum of the values recorded under `key`.
///
/// Returns `LmStatus::Absent` when `key` has no recorded values.
///
/// # Safety
/// `timing` must be a valid timing handle. `key` must be a NUL-terminated
/// UTF-8 string. `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_total_for(
    timing: *const LmTiming,
    key: *const c_char,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let key = unsafe { str_from_c(key, "key") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = timing
            .total_for(key)
            .ok_or_else(|| CapiError::new(LmStatus::Absent, "timing has no values for key"))?;
        Ok(())
    })
}

/// Overwrites the values recorded under `key` with a single `value`.
///
/// # Safety
/// `timing` must be a valid mutable timing handle. `key` must be a
/// NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_set(
    timing: *mut LmTiming,
    key: *const c_char,
    value: f64,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { mut_from_c(timing as *mut Timing, "timing") }?;
        let key = unsafe { str_from_c(key, "key") }?;
        timing.set(key, value);
        Ok(())
    })
}

/// Appends `value` to the list recorded under `key`.
///
/// # Safety
/// `timing` must be a valid mutable timing handle. `key` must be a
/// NUL-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_add(
    timing: *mut LmTiming,
    key: *const c_char,
    value: f64,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { mut_from_c(timing as *mut Timing, "timing") }?;
        let key = unsafe { str_from_c(key, "key") }?;
        timing.add(key, value);
        Ok(())
    })
}

/// Creates a new timing handle with the given total duration in seconds and
/// no recorded start/end time or sub-timings.
///
/// Returns a non-null handle in `out` that must be released with [`lm_timing_free`].
///
/// # Safety
/// `out` must point to writable `LmTiming *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_new(total: f64, out: *mut *mut LmTiming) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Timing::new(total));
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
        *unsafe { out_from_c(out) }? = handle_to_c(timing.clone());
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

/// Serializes a timing record to bytes, writing an owned buffer to `out`.
///
/// The buffer must be released with `lm_buffer_free`.
///
/// # Safety
/// `timing` must be a valid timing handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_serialize(
    timing: *const LmTiming,
    out: *mut LmBuffer,
) -> LmStatus {
    guard(|| {
        let timing = unsafe { ref_from_c(timing as *const Timing, "timing") }?;
        let out = unsafe { out_from_c(out) }?;
        let bytes = timing
            .encode(Some(true), None)
            .map_err(CapiError::serialization)?;
        *out = buffer_from_vec(bytes);
        Ok(())
    })
}

/// Deserializes a timing record from `len` bytes at `data`, writing a new handle to `out`.
///
/// The new handle must be released with [`lm_timing_free`].
///
/// # Safety
/// `data` must point to `len` readable bytes and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_timing_deserialize(
    data: *const u8,
    len: usize,
    out: *mut *mut LmTiming,
) -> LmStatus {
    guard(|| {
        let bytes = unsafe { slice_from_c(data, len, "data") }?;
        let out = unsafe { out_from_c(out) }?;
        let timing: Timing = bytes
            .unversionize()
            .decompress()
            .map_err(CapiError::deserialization)?
            .decode(())
            .map_err(CapiError::deserialization)?;
        *out = handle_to_c(timing);
        Ok(())
    })
}
