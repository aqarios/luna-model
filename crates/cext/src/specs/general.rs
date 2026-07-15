use crate::{
    buffer::buffer_from_vec,
    utils::{handle_to_c, out_from_c, ref_from_c},
};
use luna_model::types::Specs;

use crate::{
    LmBuffer, LmSpecs, LmStatus,
    error::{CapiError, guard},
};

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

/// Returns the human-readable difference between two specification values.
///
/// # Safety
/// `specs` and `other` must be valid handles and `out` writable buffer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_specs_diff(
    specs: *const LmSpecs,
    other: *const LmSpecs,
    out: *mut LmBuffer,
) -> LmStatus {
    guard(|| {
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let other = unsafe { ref_from_c(other as *const Specs, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = buffer_from_vec(specs.diff(other).map_err(CapiError::errored)?.into_bytes());
        Ok(())
    })
}
