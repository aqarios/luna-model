//! Owned index-map-item-buffer helpers for the C ABI.
#![allow(dead_code)]

use std::ffi::CString;

use lunamodel_core::solution::Column;

use crate::{LmIndexMapItem, LmIndexMapItemBuffer, utils::handle_to_c};

/// An empty index-map-item buffer.
pub(crate) fn empty_index_map_item_buffer() -> LmIndexMapItemBuffer {
    LmIndexMapItemBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts owned `(name, column)` pairs into a buffer the caller now owns.
///
/// Each name becomes a heap-allocated NUL-terminated C string and each column an
/// owned [`LmColumn`] handle. Entries must be supplied in map (column) order; the
/// enumeration position is stored as each item's `index`.
pub(crate) fn index_map_item_buffer_from_vec(items: Vec<(String, Column)>) -> LmIndexMapItemBuffer {
    if items.is_empty() {
        return empty_index_map_item_buffer();
    }

    let mut vec: Vec<LmIndexMapItem> = items
        .into_iter()
        .enumerate()
        .map(|(index, (name, col))| LmIndexMapItem {
            index,
            key: CString::new(name).unwrap_or_default().into_raw(),
            value: handle_to_c(col),
        })
        .collect();

    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);

    LmIndexMapItemBuffer { data, len, cap }
}

/// Frees an index-map-item buffer previously returned by LunaModel.
///
/// This frees the entry array plus each owned key string and column handle.
///
/// A null/empty buffer is a no-op. Each buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_index_map_buffer_free(buffer: LmIndexMapItemBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }

    let items = unsafe { Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap) };

    for item in items {
        if !item.key.is_null() {
            unsafe { drop(CString::from_raw(item.key)) };
        }
        if !item.value.is_null() {
            unsafe { drop(Box::from_raw(item.value as *mut Column)) };
        }
    }
}
