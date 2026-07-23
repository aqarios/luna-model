use lunamodel_core::prelude::Linear;

use crate::{
    LmLinear, LmStatus,
    error::guard,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Compares two linear terms for equality.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `other` must be a valid linear handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_eq(
    linear: *const LmLinear,
    other: *const LmLinear,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { ref_from_c(linear as *const Linear, "linear") }?;
        let other = unsafe { ref_from_c(other as *const Linear, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = linear.eq(other);
        Ok(())
    })
}

/// Clones a linear term into a new, independent handle written to `out`.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `LmLinear *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_clone(
    linear: *const LmLinear,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { ref_from_c(linear as *const Linear, "linear") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(linear.clone());
        Ok(())
    })
}
