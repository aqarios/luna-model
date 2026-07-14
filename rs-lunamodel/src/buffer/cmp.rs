use lunamodel_cext_vtable::LmCmpBuffer;
use lunamodel_types::Comparator;

use crate::{
    Result,
    api::api,
    traits::{LmFrom, LmTryInto, TryToVec},
};

impl TryToVec<Comparator> for LmCmpBuffer {
    fn try_to_vec(self) -> Result<Vec<Comparator>> {
        let values = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .copied()
                .map(Comparator::lm_from)
                .collect()
        };
        unsafe { (api().cmp_buffer_free)(self) };
        Ok(values)
    }
}

impl LmTryInto<Vec<Comparator>> for LmCmpBuffer {
    fn lm_try_into(self) -> Result<Vec<Comparator>> {
        self.try_to_vec()
    }
}
