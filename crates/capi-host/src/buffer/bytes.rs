//! Owned byte-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::LmBuffer;

/// An empty buffer.
pub(crate) fn empty_buffer() -> LmBuffer {
    LmBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<u8>` into a buffer the caller now owns.
pub(crate) fn buffer_from_vec(mut vec: Vec<u8>) -> LmBuffer {
    if vec.is_empty() {
        return empty_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmBuffer { data, len, cap }
}

/// Frees a buffer previously returned by LunaModel.
///
/// A null/empty buffer is a no-op. Each buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_buffer_free(buffer: LmBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap)) };
}
