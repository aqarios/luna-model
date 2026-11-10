use lunamodel_capi::LmColumnBuffer;

use crate::{Column, Result, api::api, traits::LmTryFrom};

impl LmTryFrom<LmColumnBuffer> for Vec<Option<Column>> {
    fn lm_try_from(buffer: LmColumnBuffer) -> Result<Self> {
        if buffer.data.is_null() {
            return Ok(Vec::new());
        }
        let handles = unsafe { std::slice::from_raw_parts_mut(buffer.data, buffer.len) };
        let mut columns = Vec::with_capacity(handles.len());
        for handle in handles {
            let raw = *handle;
            *handle = std::ptr::null_mut();
            columns.push(if raw.is_null() {
                None
            } else {
                Some(Column::from_raw(raw))
            });
        }
        unsafe { (api().column_buffer_free)(buffer) };
        Ok(columns)
    }
}
