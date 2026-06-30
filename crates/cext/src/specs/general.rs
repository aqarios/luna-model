use luna_model::types::Specs;

use crate::{
    LmSpecs, LmStatus,
    error::{CapiError, guard},
};

/// Returns whether `specs` satisfies all requirements expressed by `other`.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_satisfies(
    specs: *const LmSpecs,
    other: *const LmSpecs,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { (specs as *const Specs).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("specs"))?;
        let other = unsafe { (other as *const Specs).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = specs.satisfies(other);
        Ok(())
    })
}
