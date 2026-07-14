use lunamodel_cext_vtable::LmResultViewBuffer;

use crate::{
    Result, ResultView,
    api::api,
    traits::{LmTryInto, TryToVec},
};

impl TryToVec<ResultView> for LmResultViewBuffer {
    fn try_to_vec(self) -> Result<Vec<ResultView>> {
        let out = (|| {
            if self.data.is_null() {
                return Ok(Vec::new());
            }

            // The buffer owns each result-view handle, and
            // `result_view_buffer_free` would free them. Take each into an owning
            // `ResultView` and null the slot so the free releases only the array
            // (freeing a handle we now own would be a double-free).
            let mut out = Vec::with_capacity(self.len);
            for slot in unsafe { std::slice::from_raw_parts_mut(self.data, self.len) } {
                let view = (*slot).lm_try_into()?;
                *slot = std::ptr::null_mut();
                out.push(view);
            }
            Ok(out)
        })();
        unsafe { (api().result_view_buffer_free)(self) };
        out
    }
}

impl LmTryInto<Vec<ResultView>> for LmResultViewBuffer {
    fn lm_try_into(self) -> Result<Vec<ResultView>> {
        self.try_to_vec()
    }
}
