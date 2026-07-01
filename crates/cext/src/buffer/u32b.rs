//! Owned ctype-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::LmU32Buffer;

/// An empty U32Item buffer.
pub(crate) fn empty_u32_buffer() -> LmU32Buffer {
    LmU32Buffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<LmU32Item>` into a buffer the caller now owns.
pub(crate) fn u32_buffer_from_vec(mut vec: Vec<u32>) -> LmU32Buffer {
    if vec.is_empty() {
        return empty_u32_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmU32Buffer { data, len, cap }
}

/// Frees a u32 item buffer previously returned by LunaModel.
///
/// A null/empty buffer is a no-op. Each ctype buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_u32_buffer_free(buffer: LmU32Buffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap)) };
}
