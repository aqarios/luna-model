use std::ffi::CStr;

use lunamodel_cext_vtable::LmStrBuffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<String> for LmStrBuffer {
    fn try_to_vec(self) -> Result<Vec<String>> {
        let out = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .map(|&s| {
                    if s.is_null() {
                        String::new()
                    } else {
                        unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned()
                    }
                })
                .collect()
        };
        // Frees the array and every owned C string; we only copied them, so
        // nothing is double-freed.
        unsafe { (api().str_buffer_free)(self) };
        Ok(out)
    }
}

impl LmTryInto<Vec<String>> for LmStrBuffer {
    fn lm_try_into(self) -> Result<Vec<String>> {
        self.try_to_vec()
    }
}
