use luna_model::core::prelude::Bounds;

use crate::{
    LmBound, LmBounds, LmStatus, LmVtype,
    error::{CapiError, guard},
    utils::{bound_from_c, vtype_from_c},
};

/// Frees a bounds handle. Passing null is a no-op.
///
/// # Safety
/// `bounds` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_free(bounds: *mut LmBounds) {
    if !bounds.is_null() {
        drop(unsafe { Box::from_raw(bounds as *mut Bounds) });
    }
}

/// Creates a new bounds.
///
/// # Safety
/// `lower`  must be a valid bound handle created by this library.
/// `upper`  must be a valid bound handle created by this library.
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_new(
    lower: LmBound,
    upper: LmBound,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let bounds = Bounds::new(bound_from_c(lower), bound_from_c(upper));
        *out = Box::into_raw(Box::new(bounds)) as *mut LmBounds;
        Ok(())
    })
}

/// Creates new binary bounds.
///
/// # Safety
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_binary(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(Bounds::binary())) as *mut LmBounds;
        Ok(())
    })
}

/// Creates new spin bounds.
///
/// # Safety
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_spin(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(Bounds::spin())) as *mut LmBounds;
        Ok(())
    })
}

/// Creates new integer bounds.
///
/// # Safety
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_integer(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(Bounds::integer())) as *mut LmBounds;
        Ok(())
    })
}

/// Creates new real bounds.
///
/// # Safety
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_real(out: *mut *mut LmBounds) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(Bounds::real())) as *mut LmBounds;
        Ok(())
    })
}

/// Creates new bounds for the specified vtype.
///
/// # Safety
/// `vtype` the vtype to generate the bounds for.
/// `out`   must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_default_for(
    vtype: LmVtype,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(Bounds::default_for(&vtype_from_c(vtype)))) as *mut LmBounds;
        Ok(())
    })
}
