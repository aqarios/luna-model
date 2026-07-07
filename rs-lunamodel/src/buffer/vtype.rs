use lunamodel_cext_vtable::LmVtypeBuffer;
use lunamodel_types::Vtype;

use crate::{
    Result,
    api::api,
    traits::{LmFrom, LmTryInto, TryToVec},
};

impl TryToVec<Vtype> for LmVtypeBuffer {
    fn try_to_vec(self) -> Result<Vec<Vtype>> {
        let out = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .copied()
                .map(Vtype::lm_from)
                .collect()
        };
        unsafe { (api().vtype_buffer_free)(self) };
        Ok(out)
    }
}

impl LmTryInto<Vec<Vtype>> for LmVtypeBuffer {
    fn lm_try_into(self) -> Result<Vec<Vtype>> {
        self.try_to_vec()
    }
}
