//! Owned bool-map-buffer helpers for the C ABI.
#![allow(dead_code)]

use std::ffi::CString;

use crate::{LmBoolMapBuffer, LmBoolMapEntry};

/// An empty bool-map buffer.
pub(crate) fn empty_bool_map_buffer() -> LmBoolMapBuffer {
    LmBoolMapBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts owned `(name, flags)` pairs into a buffer the caller now owns.
///
/// Each name becomes a heap-allocated NUL-terminated C string and each flag
/// series an owned `bool` array.
pub(crate) fn bool_map_buffer_from_vec(entries: Vec<(String, Vec<bool>)>) -> LmBoolMapBuffer {
    if entries.is_empty() {
        return empty_bool_map_buffer();
    }

    let mut vec: Vec<LmBoolMapEntry> = entries
        .into_iter()
        .map(|(name, mut flags)| {
            let (data, len, cap) = (flags.as_mut_ptr(), flags.len(), flags.capacity());
            std::mem::forget(flags);
            LmBoolMapEntry {
                key: CString::new(name).unwrap_or_default().into_raw(),
                data,
                len,
                cap,
            }
        })
        .collect();

    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);

    LmBoolMapBuffer { data, len, cap }
}

/// Frees a bool-map buffer previously returned by LunaModel.
///
/// This frees the entry array plus each owned key string and flag array.
///
/// A null/empty buffer is a no-op. Each buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bool_map_buffer_free(buffer: LmBoolMapBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }

    let entries = unsafe { Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap) };

    for entry in entries {
        if !entry.key.is_null() {
            unsafe { drop(CString::from_raw(entry.key)) };
        }
        if !entry.data.is_null() && entry.cap != 0 {
            unsafe { drop(Vec::from_raw_parts(entry.data, entry.len, entry.cap)) };
        }
    }
}
