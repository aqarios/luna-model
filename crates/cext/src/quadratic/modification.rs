use crate::utils::{handle_to_c, mut_from_c, out_from_c, ref_from_c};
use std::ops::{AddAssign, MulAssign, Neg};

use luna_model::core::prelude::Quadratic;

use crate::{LmQuadratic, LmStatus, error::guard};

/// Adds a new entry with its bias to the quadratic term.
///
/// # Safety
/// `quadratic` must be a valid mutable quadratic handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_add_entry(
    quadratic: *mut LmQuadratic,
    u: u32,
    v: u32,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { mut_from_c(quadratic as *mut Quadratic, "quadratic") }?;
        quadratic.add_assign((u, v, bias));
        Ok(())
    })
}

/// Adds another quadratic term to this quadratic term.
///
/// # Safety
/// `quadratic` must be a valid mutable quadratic handle created by this library.
/// `other` must be a valid quadratic handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_add_quadratic(
    quadratic: *mut LmQuadratic,
    other: *const LmQuadratic,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { mut_from_c(quadratic as *mut Quadratic, "quadratic") }?;
        let other = unsafe { ref_from_c(other as *const Quadratic, "other") }?;
        quadratic.add_assign(other);
        Ok(())
    })
}

/// Multiplies every bias of this quadratic term by `bias`.
///
/// # Safety
/// `quadratic` must be a valid mutable quadratic handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_mul_bias(quadratic: *mut LmQuadratic, bias: f64) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { mut_from_c(quadratic as *mut Quadratic, "quadratic") }?;
        quadratic.mul_assign(bias);
        Ok(())
    })
}

/// Negates this quadratic term, writing the result to a new quadratic handle.
///
/// On success, `out` receives an owned handle that must be released with
/// [`lm_quadratic_free`].
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `LmQuadratic *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_neg(
    quadratic: *const LmQuadratic,
    out: *mut *mut LmQuadratic,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(quadratic.neg());
        Ok(())
    })
}

/// Sets the bias for a specific index of the quadratic term.
///
/// # Safety
/// `quadratic` must be a valid mutable quadratic handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_set_bias(
    quadratic: *mut LmQuadratic,
    u: u32,
    v: u32,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { mut_from_c(quadratic as *mut Quadratic, "quadratic") }?;
        quadratic[(u, v)] = bias;
        Ok(())
    })
}
