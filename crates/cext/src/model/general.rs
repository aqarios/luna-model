use crate::error::{CapiError, guard};
use crate::{LmModel, LmStatus};
use luna_model::core::Model;

/// Clones a model into a new, independent handle written to `out`.
///
/// # Safety
/// `model` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_clone(model: *const LmModel, out: *mut *mut LmModel) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(model.clone())) as *mut LmModel;
        Ok(())
    })
}
