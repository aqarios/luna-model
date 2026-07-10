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
            // The buffer owns each sample-view handle, and
            // `sample_view_buffer_free` would free them. Take each into an owning
            // `SampleView` and null the slot so the free releases only the array
            // (freeing a handle we now own would be a double-free).
            unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
                .iter_mut()
                .map(|slot| {
                    let view = SampleView::from_raw(*slot);
                    *slot = std::ptr::null_mut();
                    view
                })
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
