//! Conversion of owned C-ABI linear-item buffers into Rust vectors.

use lunamodel_cext_vtable::LmLinearItemBuffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<(u32, f64)> for LmLinearItemBuffer {
    fn try_to_vec(self) -> Result<Vec<(u32, f64)>> {
        let items = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .map(|item| (item.idx, item.bias))
                .collect()
        };

        unsafe { (api().linear_item_buffer_free)(self) };

        Ok(items)
    }
}

impl LmTryInto<Vec<(u32, f64)>> for LmLinearItemBuffer {
    fn lm_try_into(self) -> Result<Vec<(u32, f64)>> {
        self.try_to_vec()
    }
}
