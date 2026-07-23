use lunamodel_capi::LmBuffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToString, TryToVec},
};

impl TryToString for LmBuffer {
    fn try_to_string(self) -> Result<String> {
        let bytes = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec()
        };

        unsafe { (api().buffer_free)(self) };

        String::from_utf8(bytes).map_err(|err| crate::Error {
            status: crate::Status::InvalidArgument,
            message: err.to_string(),
        })
    }
}

impl TryToVec<u8> for LmBuffer {
    fn try_to_vec(self) -> Result<Vec<u8>> {
        let bytes = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec()
        };
        unsafe { (api().buffer_free)(self) };
        Ok(bytes)
    }
}

impl LmTryInto<String> for LmBuffer {
    fn lm_try_into(self) -> Result<String> {
        self.try_to_string()
    }
}

impl LmTryInto<Vec<u8>> for LmBuffer {
    fn lm_try_into(self) -> Result<Vec<u8>> {
        self.try_to_vec()
    }
}
