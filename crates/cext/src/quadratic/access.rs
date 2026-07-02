use crate::utils::{out_from_c, ref_from_c};
use luna_model::core::prelude::Quadratic;

use crate::{
    LmQuadratic, LmQuadraticItem, LmQuadraticItemBuffer, LmStatus,
    buffer::quadratic_item_buffer_from_vec, error::guard,
};

/// Returns the number of explicitly stored entries of the quadratic term.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_len(
    quadratic: *const LmQuadratic,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = quadratic.len();
        Ok(())
    })
}

/// Returns whether the quadratic term is zero.
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
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = quadratic.is_zero();
        Ok(())
    })
}

/// Returns whether the quadratic term is empty.
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
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
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
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
        let items = quadratic
            .iter_flat()
            .map(|(u_idx, v_idx, bias)| LmQuadraticItem { u_idx, v_idx, bias })
            .collect::<Vec<_>>();
        *out = quadratic_item_buffer_from_vec(items);
        Ok(())
    })
}

/// Returns the bias stored in the quadratic term for the given variable index.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_index(
    quadratic: *const LmQuadratic,
    u: u32,
    v: u32,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = quadratic[(u, v)];
        Ok(())
    })
}
