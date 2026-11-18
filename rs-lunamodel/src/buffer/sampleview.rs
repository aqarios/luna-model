use lunamodel_capi::LmSampleViewBuffer;

use crate::{
    Result, SampleView,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<SampleView> for LmSampleViewBuffer {
    fn try_to_vec(self) -> Result<Vec<SampleView>> {
        let out = (|| {
            if self.data.is_null() {
                return Ok(Vec::new());
            }

            // The buffer owns each sample-view handle, and
            // `sample_view_buffer_free` would free them. Take each into an owning
            // `SampleView` and null the slot so the free releases only the array
            // (freeing a handle we now own would be a double-free).
            let mut out = Vec::with_capacity(self.len);
            for slot in unsafe { std::slice::from_raw_parts_mut(self.data, self.len) } {
                let view = (*slot).lm_try_into()?;
                *slot = std::ptr::null_mut();
                out.push(view);
            }
            Ok(out)
        })();
        unsafe { (api().sample_view_buffer_free)(self) };
        out
    }
}

impl LmTryInto<Vec<SampleView>> for LmSampleViewBuffer {
    fn lm_try_into(self) -> Result<Vec<SampleView>> {
        self.try_to_vec()
    }
}
