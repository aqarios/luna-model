//! An owned byte buffer handed across the C ABI.
#![allow(dead_code)]

/// An owned, heap-allocated byte buffer the caller takes ownership of.
///
/// Buffers returned by LunaModel (e.g. from `lm_model_serialize`) must be released
/// with [`lm_buffer_free`]. The fields mirror a `Vec<u8>` so it can be rebuilt for
/// deallocation.
#[repr(C)]
pub struct LmBuffer {
    /// Pointer to the first byte, or null for an empty buffer.
    pub data: *mut u8,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

impl LmBuffer {
    /// An empty buffer.
    pub(crate) fn empty() -> Self {
        Self {
            data: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    /// Converts an owned `Vec<u8>` into a buffer the caller now owns.
    pub(crate) fn from_vec(mut vec: Vec<u8>) -> Self {
        if vec.is_empty() {
            return Self::empty();
        }
        let (data, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
        std::mem::forget(vec);
        Self { data, len, cap }
    }
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
