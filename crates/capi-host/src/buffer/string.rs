//! Owned byte-buffer helpers for the C ABI.
#![allow(dead_code)]

use std::ffi::{CString, c_char};

use crate::LmStrBuffer;

/// An empty stringbuffer.
pub(crate) fn empty_str_buffer() -> LmStrBuffer {
    LmStrBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<String>` into a buffer the caller now owns.
///
/// Each string becomes heap-allocated NUL-termianted C string.
pub(crate) fn str_buffer_from_vec(vec: Vec<String>) -> LmStrBuffer {
    if vec.is_empty() {
        return empty_str_buffer();
    }

    let mut vec: Vec<*mut c_char> = vec
        .into_iter()
        .map(|s| CString::new(s).unwrap_or_default().into_raw())
        .collect();

    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmStrBuffer { data, len, cap }
}

/// Frees a string buffer previously returned by LunaModel.
///
/// This frees both the pointer array and each owned C string inside it.
///
/// A null/empty buffer is a no-op. Each string buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_str_buffer_free(buffer: LmStrBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }

    let strings = unsafe { Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap) };

    for s in strings {
        if !s.is_null() {
            unsafe { drop(CString::from_raw(s)) };
        }
    }
}
