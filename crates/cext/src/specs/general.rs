use crate::utils::{handle_to_c, out_from_c, ref_from_c};
use luna_model::types::Specs;

use crate::{LmSpecs, LmStatus, error::guard};

/// Returns whether `specs` satisfies all requirements expressed by `other`.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `other` must be a valid specs handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_satisfies(
    specs: *const LmSpecs,
    other: *const LmSpecs,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let other = unsafe { ref_from_c(other as *const Specs, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = specs.satisfies(other);
        Ok(())
    })
}

/// Clones a specs into a new, independent handle written to `out`.
///
/// # Safety
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `LmSpecs *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_clone(specs: *const LmSpecs, out: *mut *mut LmSpecs) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(specs.clone());
        Ok(())
    })
}
