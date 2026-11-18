use lunamodel_capi::LmF64Buffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<f64> for LmF64Buffer {
    fn try_to_vec(self) -> Result<Vec<f64>> {
        let values = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec()
        };
        unsafe { (api().f64_buffer_free)(self) };
        Ok(values)
    }
}

impl LmTryInto<Vec<f64>> for LmF64Buffer {
    fn lm_try_into(self) -> Result<Vec<f64>> {
        self.try_to_vec()
    }
}
