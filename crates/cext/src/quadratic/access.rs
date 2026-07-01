use luna_model::core::prelude::Quadratic;

use crate::{
    LmQuadratic, LmQuadraticItem, LmQuadraticItemBuffer, LmStatus,
    buffer::quadratic_item_buffer_from_vec,
    error::{CapiError, guard},
};

/// Returns the len of the quadratic term.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_len(
    quadratic: *const LmQuadratic,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = quadratic.len();
        Ok(())
    })
}

/// Returns if the quadratic term is zero.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_is_zero(
    quadratic: *const LmQuadratic,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = quadratic.is_zero();
        Ok(())
    })
}

/// Returns if the quadratic term is empty.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_is_empty(
    quadratic: *const LmQuadratic,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = quadratic.is_empty();
        Ok(())
    })
}

/// Returns the quadratic term items.
///
/// The returned array is owned by the caller and must be released with [`lm_quadratic_item_buffer_free`].
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `LmQuadraticItemBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_items(
    quadratic: *const LmQuadratic,
    out: *mut LmQuadraticItemBuffer,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let items = quadratic
            .iter_flat()
            .map(|(u_idx, v_idx, bias)| LmQuadraticItem { u_idx, v_idx, bias })
            .collect::<Vec<_>>();
        *out = quadratic_item_buffer_from_vec(items);
        Ok(())
    })
}

/// Returns the bias in the quadratic term for the var idx.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `f64` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_index(
    quadratic: *const LmQuadratic,
    u: u32,
    v: u32,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = quadratic[(u, v)];
        Ok(())
    })
}
