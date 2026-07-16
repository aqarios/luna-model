use std::ops::{AddAssign, MulAssign, Neg};

use luna_model::core::prelude::Linear;

use crate::{
    LmLinear, LmStatus,
    error::guard,
    utils::{handle_to_c, mut_from_c, out_from_c, ref_from_c},
};

/// Adds a new entry with its bias to the linear term.
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
        let linear = unsafe { mut_from_c(linear as *mut Linear, "linear") }?;
        linear.add_assign((id, bias));
        Ok(())
    })
}

/// Adds another linear term to this linear term.
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
        let linear = unsafe { mut_from_c(linear as *mut Linear, "linear") }?;
        let other = unsafe { ref_from_c(other as *const Linear, "other") }?;
        linear.add_assign(other);
        Ok(())
    })
}

/// Multiplies every bias of this linear term by `bias`.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_mul_bias(linear: *mut LmLinear, bias: f64) -> LmStatus {
    guard(|| {
        let linear = unsafe { mut_from_c(linear as *mut Linear, "linear") }?;
        linear.mul_assign(bias);
        Ok(())
    })
}

/// Negates this linear term, writing the result to a new linear handle.
///
/// On success, `out` receives an owned handle that must be released with
/// [`lm_linear_free`].
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `LmLinear *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_neg(
    linear: *const LmLinear,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { ref_from_c(linear as *const Linear, "linear") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(linear.neg());
        Ok(())
    })
}

/// Sets the bias for a specific index of the linear term.
///
/// # Safety
/// `linear` must be a valid mutable linear handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_set_bias(linear: *mut LmLinear, id: u32, bias: f64) -> LmStatus {
    guard(|| {
        let linear = unsafe { mut_from_c(linear as *mut Linear, "linear") }?;
        linear[id] = bias;
        Ok(())
    })
}
