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
            // The buffer owns each vref handle, and `vref_buffer_free` would free
            // them. Take each into an owning `VarRef` and null the slot so the
            // free releases only the array (freeing a handle we now own would be
            // a double-free).
            unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
                .iter_mut()
                .map(|slot| {
                    let vref = VarRef::from_raw(*slot);
                    *slot = std::ptr::null_mut();
                    vref
                })
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
