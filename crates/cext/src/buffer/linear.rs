//! Owned linear-item-buffer helpers for the C ABI.
#![allow(dead_code)]

use crate::{LmLinearItem, LmLinearItemBuffer};

/// An empty LinearItem buffer.
pub(crate) fn empty_linear_item_buffer() -> LmLinearItemBuffer {
    LmLinearItemBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<LmLinearItem>` into a buffer the caller now owns.
pub(crate) fn linear_item_buffer_from_vec(mut vec: Vec<LmLinearItem>) -> LmLinearItemBuffer {
    if vec.is_empty() {
        return empty_linear_item_buffer();
    }
    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);
    LmLinearItemBuffer { data, len, cap }
}

/// Frees a linear item buffer previously returned by LunaModel.
///
/// A null/empty buffer is a no-op. Each linear item buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_item_buffer_free(buffer: LmLinearItemBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap)) };
}
