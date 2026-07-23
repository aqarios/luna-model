//! Owned f64-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::LmF64Buffer;

/// An empty f64 buffer.
pub(crate) fn empty_f64_buffer() -> LmF64Buffer {
    LmF64Buffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<f64>` into a buffer the caller now owns.
pub(crate) fn f64_buffer_from_vec(mut vec: Vec<f64>) -> LmF64Buffer {
    if vec.is_empty() {
        return empty_f64_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmF64Buffer { data, len, cap }
}

/// Frees an f64 buffer previously returned by LunaModel.
///
/// A null/empty buffer is a no-op. Each f64 buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_f64_buffer_free(buffer: LmF64Buffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap)) };
}
