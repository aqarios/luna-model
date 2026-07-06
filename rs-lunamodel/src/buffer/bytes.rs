use lunamodel_cext_vtable::LmBuffer;

use crate::{
    Result,
    api::api,
    traits::{TryToString, TryToVec},
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
