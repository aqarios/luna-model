//! Owned vref-buffer helpers for the C ABI.
#![allow(dead_code)]

use luna_model::core::prelude::VarRef;

use crate::{LmVref, LmVrefBuffer};

/// An empty vref buffer.
pub(crate) fn empty_vref_buffer() -> LmVrefBuffer {
    LmVrefBuffer {
        data: std::ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

/// Converts an owned `Vec<LmVref *>` into a buffer the caller now owns.
pub(crate) fn vref_buffer_from_vec(mut vec: Vec<*mut LmVref>) -> LmVrefBuffer {
    if vec.is_empty() {
        return empty_vref_buffer();
    }

    let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
    std::mem::forget(vec);

    LmVrefBuffer { data, len, cap }
}

/// Frees a vref buffer previously returned by LunaModel.
///
/// This frees both the pointer array and each owned variable handle inside it.
///
/// A null/empty buffer is a no-op. Each vref buffer must be freed at most once.
///
/// # Safety
/// `buffer` must have been produced by this library and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_vref_buffer_free(buffer: LmVrefBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }

    let vars = unsafe { Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap) };

    for var in vars {
        if !var.is_null() {
            unsafe {
                drop(Box::from_raw(var as *mut VarRef));
            }
        }
    }
}
