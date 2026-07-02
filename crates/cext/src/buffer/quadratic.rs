//! Owned quadratic-item-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::{LmQuadraticItem, LmQuadraticItemBuffer};

/// An empty QuadraticItem buffer.
pub(crate) fn empty_quadratic_item_buffer() -> LmQuadraticItemBuffer {
    LmQuadraticItemBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<LmQuadraticItem>` into a buffer the caller now owns.
pub(crate) fn quadratic_item_buffer_from_vec(
    mut vec: Vec<LmQuadraticItem>,
) -> LmQuadraticItemBuffer {
    if vec.is_empty() {
        return empty_quadratic_item_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmQuadraticItemBuffer { data, len, cap }
}

/// Frees a quadratic item buffer previously returned by LunaModel.
///
/// A null/empty buffer is a no-op. Each quadratic item buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_item_buffer_free(buffer: LmQuadraticItemBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap)) };
}
