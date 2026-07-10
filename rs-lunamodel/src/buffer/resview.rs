use lunamodel_cext_vtable::LmResultViewBuffer;

use crate::{
    Result, ResultView,
    api::api,
    traits::{LmTryFrom, LmTryInto, TryToVec},
};

impl TryToVec<ResultView> for LmResultViewBuffer {
    fn try_to_vec(self) -> Result<Vec<ResultView>> {
        let out = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .copied()
                .map(ResultView::from_raw)
                .collect()
        };
        unsafe { (api().result_view_buffer_free)(self) };
        Ok(out)
    }
}

impl LmTryInto<Vec<ResultView>> for LmResultViewBuffer {
    fn lm_try_into(self) -> Result<Vec<ResultView>> {
        self.try_to_vec()
    }
}

impl LmTryFrom<LmResultViewBuffer> for Option<Vec<ResultView>> {
    fn lm_try_from(value: LmResultViewBuffer) -> Result<Self> {
        todo!()
    }
}
