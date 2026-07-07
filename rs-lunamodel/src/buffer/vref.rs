use lunamodel_cext_vtable::LmVrefBuffer;

use crate::{
    Result, VarRef,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<VarRef> for LmVrefBuffer {
    fn try_to_vec(self) -> Result<Vec<VarRef>> {
        let out = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .copied()
                .map(VarRef::from_raw)
                .collect()
        };
        unsafe { (api().vref_buffer_free)(self) };
        Ok(out)
    }
}

impl LmTryInto<Vec<VarRef>> for LmVrefBuffer {
    fn lm_try_into(self) -> Result<Vec<VarRef>> {
        self.try_to_vec()
    }
}
