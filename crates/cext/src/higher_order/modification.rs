use std::ops::{AddAssign, MulAssign, Neg};

use luna_model::core::prelude::HigherOrder;

use crate::{
    LmHigherOrder, LmStatus,
    error::{CapiError, guard},
};

/// Add a new entry with it's bias to the higher_order term.
///
/// # Safety
/// `higher_order` must be a valid mutable higher_order handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_add_entry(
    higher_order: *mut LmHigherOrder,
    indices: *const u32,
    indices_len: usize,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *mut HigherOrder).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;

        // `from_raw_parts` requires a non-null pointer, so treat empty as no indices.
        let indices: &[u32] = if indices.is_null() || indices_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(indices, indices_len) }
        };

        higher_order.add_assign((indices, bias));
        Ok(())
    })
}

/// Add another higher_order term to this higher_order term.
///
/// # Safety
/// `higher_order` must be a valid mutable higher_order handle created by this library.
/// `other` must be a valid higher_order handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_add_higher_order(
    higher_order: *mut LmHigherOrder,
    other: *const LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *mut HigherOrder).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let other = unsafe { (other as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        higher_order.add_assign(other);
        Ok(())
    })
}

/// Mul this higher_order term with some value.
///
/// # Safety
/// `higher_order` must be a valid mutable higher_order handle created by this library.
/// `bias` must be a valid f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_mul_bias(
    higher_order: *mut LmHigherOrder,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *mut HigherOrder).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        higher_order.mul_assign(bias);
        Ok(())
    })
}

/// Negate this higher_order term and writing the result to a new higher_order handle.
///
/// # Safety
/// `higher_order` must be a valid mutable higher_order handle created by this library.
/// `out` must point to writable `LmHigherOrder` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_neg(
    higher_order: *const LmHigherOrder,
    out: *mut *mut LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(higher_order.neg().clone())) as *mut LmHigherOrder;
        Ok(())
    })
}

/// Set the bias for a specific entry (variable tuple) of the higher-order term.
///
/// # Safety
/// `higher_order` must be a valid mutable higher-order handle created by this library.
/// `indices` must point to `indices_len` readable `u32`s (may be null iff `indices_len == 0`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_set_bias(
    higher_order: *mut LmHigherOrder,
    indices: *const u32,
    indices_len: usize,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *mut HigherOrder).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;

        // `from_raw_parts` requires a non-null pointer, so treat empty as no indices.
        let indices: &[u32] = if indices.is_null() || indices_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(indices, indices_len) }
        };

        higher_order[indices] = bias;
        Ok(())
    })
}
