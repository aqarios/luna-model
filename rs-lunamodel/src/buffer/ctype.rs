use enumset::EnumSet;
use lunamodel_capi::LmCtypeBuffer;
use lunamodel_types::{Ctype, EnumSetFromVec};

use crate::{
    Result,
    api::api,
    traits::{LmFrom, LmTryInto, TryToVec},
};

impl TryToVec<Ctype> for LmCtypeBuffer {
    fn try_to_vec(self) -> Result<Vec<Ctype>> {
        let out = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .copied()
                .map(Ctype::lm_from)
                .collect()
        };
        unsafe { (api().ctype_buffer_free)(self) };
        Ok(out)
    }
}

impl LmTryInto<Vec<Ctype>> for LmCtypeBuffer {
    fn lm_try_into(self) -> Result<Vec<Ctype>> {
        self.try_to_vec()
    }
}

// 1. the trait bound `EnumSet<Ctype>: LmTryFrom<LmCtypeBuffer>` is not satisfied
impl LmTryInto<EnumSet<Ctype>> for LmCtypeBuffer {
    fn lm_try_into(self) -> Result<EnumSet<Ctype>> {
        Ok(self.try_to_vec()?.to_enumset())
    }
}
