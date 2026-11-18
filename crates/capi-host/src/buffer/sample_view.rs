//! Owned higher-order-item-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::{LmSampleView, LmSampleViewBuffer, sample::lm_sample_view_free};

/// An empty HigherOrderItem buffer.
pub(crate) fn empty_sample_view_buffer() -> LmSampleViewBuffer {
    LmSampleViewBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<LmHigherOrderItem>` into a buffer the caller now owns.
pub(crate) fn sample_view_buffer_from_vec(mut vec: Vec<*mut LmSampleView>) -> LmSampleViewBuffer {
    if vec.is_empty() {
        return empty_sample_view_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmSampleViewBuffer { data, len, cap }
}

/// Frees a higher-order item buffer previously returned by LunaModel.
///
/// Also frees each item's owned `indices` buffer. A null/empty buffer is a
/// no-op. Each buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sample_view_buffer_free(buffer: LmSampleViewBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    // Consume the outer Vec by value so we can move each item's inline buffer
    // into `lm_u32_buffer_free`. The outer array is freed when `items` drops.
    let items = unsafe { Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap) };
    for item in items {
        if !item.is_null() {
            unsafe { lm_sample_view_free(item) };
        }
    }
}
