use crate::utils::{drop_from_c, handle_to_c, out_from_c};
use lunamodel_core::Bounds;

use crate::{
    LmBound, LmBounds, LmStatus, LmVtype,
    error::guard,
    utils::{bound_from_c, vtype_from_c},
};

/// Frees a bounds handle. Passing null is a no-op.
///
/// # Safety
/// `bounds` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_free(bounds: *mut LmBounds) {
    unsafe { drop_from_c(bounds as *mut Bounds) }
}

/// Creates new bounds from explicit lower and upper bounds.
///
/// On success, `out` receives an owned handle that must be released with
/// [`lm_bounds_free`].
///
/// # Safety
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_new(
    lower: LmBound,
    upper: LmBound,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;

        let bounds = Bounds::new(bound_from_c(lower), bound_from_c(upper));
        *out = handle_to_c(bounds);
        Ok(())
    })
}

/// Creates new binary bounds.
///
/// # Safety
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_binary(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Bounds::binary());
        Ok(())
    })
}

/// Creates new spin bounds.
///
/// # Safety
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_spin(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Bounds::spin());
        Ok(())
    })
}

/// Creates new integer bounds.
///
/// # Safety
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_integer(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Bounds::integer());
        Ok(())
    })
}

/// Creates new real bounds.
///
/// # Safety
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_real(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Bounds::real());
        Ok(())
    })
}

/// Creates new bounds for the specified vtype.
///
/// # Safety
/// `vtype` the vtype to generate the bounds for.
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_default_for(
    vtype: LmVtype,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(Bounds::default_for(&vtype_from_c(vtype)));
        Ok(())
    })
}
