//! Conversion of owned C-ABI linear-item buffers into Rust vectors.

use lunamodel_cext_vtable::LmQuadraticItemBuffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<(u32, u32, f64)> for LmQuadraticItemBuffer {
    fn try_to_vec(self) -> Result<Vec<(u32, u32, f64)>> {
        let items = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .map(|item| (item.u_idx, item.v_idx, item.bias))
                .collect()
        };

        unsafe { (api().quadratic_item_buffer_free)(self) };

        Ok(items)
    }
}

impl LmTryInto<Vec<(u32, u32, f64)>> for LmQuadraticItemBuffer {
    fn lm_try_into(self) -> Result<Vec<(u32, u32, f64)>> {
        self.try_to_vec()
    }
}
