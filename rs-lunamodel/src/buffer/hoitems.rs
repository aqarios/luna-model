//! Conversion of owned C-ABI higher-order-item buffers into Rust vectors.

use lunamodel_cext_vtable::LmHigherOrderItemBuffer;

use crate::{
    Result,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<(Vec<u32>, f64)> for LmHigherOrderItemBuffer {
    fn try_to_vec(self) -> Result<Vec<(Vec<u32>, f64)>> {
        let items = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .map(|item| {
                    let indices = if item.indices.data.is_null() {
                        Vec::new()
                    } else {
                        unsafe { std::slice::from_raw_parts(item.indices.data, item.indices.len) }
                            .to_vec()
                    };

                    (indices, item.bias)
                })
                .collect()
        };

        // This releases the outer item array and every nested indices buffer.
        unsafe { (api().higher_order_item_buffer_free)(self) };

        Ok(items)
    }
}

impl LmTryInto<Vec<(Vec<u32>, f64)>> for LmHigherOrderItemBuffer {
    fn lm_try_into(self) -> Result<Vec<(Vec<u32>, f64)>> {
        self.try_to_vec()
    }
}
