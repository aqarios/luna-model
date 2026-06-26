//! Owned vtype-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::{LmVtype, LmVtypeBuffer};

/// An empty vtype buffer.
pub(crate) fn empty_vtype_buffer() -> LmVtypeBuffer {
    LmVtypeBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<u8>` into a buffer the caller now owns.
pub(crate) fn vtype_buffer_from_vec(mut vec: Vec<LmVtype>) -> LmVtypeBuffer {
    if vec.is_empty() {
        return empty_vtype_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmVtypeBuffer { data, len, cap }
}

/// Frees a vtype buffer previously returned by LunaModel.
///
/// A null/empty buffer is a no-op. Each vtype buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vtype_buffer_free(buffer: LmVtypeBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap)) };
}
