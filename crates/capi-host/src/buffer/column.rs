use crate::{LmColumn, LmColumnBuffer};

pub(crate) fn column_buffer_from_vec(mut values: Vec<*mut LmColumn>) -> LmColumnBuffer {
    if values.is_empty() {
        return LmColumnBuffer::default();
    }
    let buffer = LmColumnBuffer {
        data: values.as_mut_ptr(),
        len: values.len(),
        cap: values.capacity(),
    };
    std::mem::forget(values);
    buffer
}

/// Releases an owned optional-column buffer and every non-null handle in it.
///
/// # Safety
/// `buffer` must have been returned by LunaModel and not previously freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_column_buffer_free(buffer: LmColumnBuffer) {
    if buffer.data.is_null() || buffer.cap == 0 {
        return;
    }
    let values = unsafe { Vec::from_raw_parts(buffer.data, buffer.len, buffer.cap) };
    for value in values {
        if !value.is_null() {
            unsafe { crate::col::lm_col_free(value) };
        }
    }
}
