use std::ops::{AddAssign, MulAssign, Neg};

use luna_model::core::prelude::Linear;

use crate::{
    LmLinear, LmStatus,
    error::{CapiError, guard},
};

/// Add a new entry with it's bias to the linear term.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_add_entry(
    linear: *mut LmLinear,
    id: u32,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *mut Linear).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        linear.add_assign((id, bias));
        Ok(())
    })
}

/// Add another linear term to this linear term.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
/// `other` must be a valid linear handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_add_linear(
    linear: *mut LmLinear,
    other: *const LmLinear,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *mut Linear).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let other = unsafe { (other as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        linear.add_assign(other);
        Ok(())
    })
}

/// Mul this linear term with some value.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
/// `bias` must be a valid f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_mul_bias(linear: *mut LmLinear, bias: f64) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *mut Linear).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        linear.mul_assign(bias);
        Ok(())
    })
}

/// Negate this linear term and writing the result to a new linear handle.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
/// `out` must point to writable `LmLinear` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_neg(
    linear: *const LmLinear,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(linear.neg().clone())) as *mut LmLinear;
        Ok(())
    })
}

/// Set bias for a specific index of the linear term.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_set_bias(linear: *mut LmLinear, id: u32, bias: f64) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *mut Linear).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        linear[id] = bias;
        Ok(())
    })
}
