use lunamodel_capi::LmU32Buffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<u32> for LmU32Buffer {
    fn try_to_vec(self) -> Result<Vec<u32>> {
        let values = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec()
        };
        unsafe { (api().u32_buffer_free)(self) };
        Ok(values)
    }
}

impl LmTryInto<Vec<u32>> for LmU32Buffer {
    fn lm_try_into(self) -> Result<Vec<u32>> {
        self.try_to_vec()
    }
}
