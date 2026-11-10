use std::{collections::HashMap, ffi::CStr};

use lunamodel_capi::LmBoolMapBuffer;

use crate::{Result, api::api, traits::LmTryInto};

impl LmTryInto<HashMap<String, Vec<bool>>> for LmBoolMapBuffer {
    fn lm_try_into(self) -> Result<HashMap<String, Vec<bool>>> {
        let mut values = HashMap::with_capacity(self.len);
        if !self.data.is_null() {
            for entry in unsafe { std::slice::from_raw_parts(self.data, self.len) } {
                let key = if entry.key.is_null() {
                    String::new()
                } else {
                    unsafe { CStr::from_ptr(entry.key) }
                        .to_string_lossy()
                        .into_owned()
                };
                let flags = if entry.data.is_null() {
                    Vec::new()
                } else {
                    unsafe { std::slice::from_raw_parts(entry.data, entry.len) }.to_vec()
                };
                values.insert(key, flags);
            }
        }
        unsafe { (api().bool_map_buffer_free)(self) };
        Ok(values)
    }
}
