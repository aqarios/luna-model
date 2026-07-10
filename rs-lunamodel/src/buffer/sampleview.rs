use lunamodel_cext_vtable::LmSampleViewBuffer;

use crate::{
    Result, SampleView,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<SampleView> for LmSampleViewBuffer {
    fn try_to_vec(self) -> Result<Vec<SampleView>> {
        let out = if self.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.len) }
                .iter()
                .copied()
                .map(SampleView::from_raw)
                .collect()
        };
        unsafe { (api().sample_view_buffer_free)(self) };
        Ok(out)
    }
}

impl LmTryInto<Vec<SampleView>> for LmSampleViewBuffer {
    fn lm_try_into(self) -> Result<Vec<SampleView>> {
        self.try_to_vec()
    }
}
