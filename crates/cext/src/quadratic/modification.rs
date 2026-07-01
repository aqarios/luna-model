use std::ops::{AddAssign, MulAssign, Neg};

use luna_model::core::prelude::Quadratic;

use crate::{
    LmQuadratic, LmStatus,
    error::{CapiError, guard},
};

/// Add a new entry with it's bias to the quadratic term.
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
        let quadratic = unsafe { (quadratic as *mut Quadratic).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        quadratic.add_assign((u, v, bias));
        Ok(())
    })
}

/// Add another quadratic term to this quadratic term.
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
        let quadratic = unsafe { (quadratic as *mut Quadratic).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let other = unsafe { (other as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        quadratic.add_assign(other);
        Ok(())
    })
}

/// Mul this quadratic term with some value.
///
/// # Safety
/// `quadratic` must be a valid mutable quadratic handle created by this library.
/// `bias` must be a valid f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_mul_bias(quadratic: *mut LmQuadratic, bias: f64) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *mut Quadratic).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        quadratic.mul_assign(bias);
        Ok(())
    })
}

/// Negate this quadratic term and writing the result to a new quadratic handle.
///
/// # Safety
/// `quadratic` must be a valid mutable quadratic handle created by this library.
/// `out` must point to writable `LmQuadratic` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_neg(
    quadratic: *const LmQuadratic,
    out: *mut *mut LmQuadratic,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(quadratic.neg().clone())) as *mut LmQuadratic;
        Ok(())
    })
}

/// Set bias for a specific index of the quadratic term.
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
        let quadratic = unsafe { (quadratic as *mut Quadratic).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        quadratic[(u, v)] = bias;
        Ok(())
    })
}
