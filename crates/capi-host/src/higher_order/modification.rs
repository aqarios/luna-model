use std::{
    ffi::c_char,
    ops::{AddAssign, MulAssign, Neg},
};

use lunamodel_core::prelude::HigherOrder;

use crate::{
    LmHigherOrder, LmStatus,
    error::guard,
    utils::{handle_to_c, mut_from_c, opt_slice_from_c, out_from_c, ref_from_c, str_from_c},
};

/// Adds a new entry with its bias to the higher_order term.
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
        let higher_order = unsafe { mut_from_c(higher_order as *mut HigherOrder, "higher_order") }?;

        let indices = unsafe { opt_slice_from_c(indices, indices_len) }.unwrap_or(&[]);

        higher_order.add_assign((indices, bias));
        Ok(())
    })
}

/// Adds another higher_order term to this higher_order term.
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
        let higher_order = unsafe { mut_from_c(higher_order as *mut HigherOrder, "higher_order") }?;
        let other = unsafe { ref_from_c(other as *const HigherOrder, "other") }?;
        higher_order.add_assign(other);
        Ok(())
    })
}

/// Multiplies every bias of this higher_order term by `bias`.
///
/// # Safety
/// `higher_order` must be a valid mutable higher_order handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_mul_bias(
    higher_order: *mut LmHigherOrder,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { mut_from_c(higher_order as *mut HigherOrder, "higher_order") }?;
        higher_order.mul_assign(bias);
        Ok(())
    })
}

/// Negates this higher_order term, writing the result to a new higher_order handle.
///
/// On success, `out` receives an owned handle that must be released with
/// [`lm_higher_order_free`].
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `LmHigherOrder *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_neg(
    higher_order: *const LmHigherOrder,
    out: *mut *mut LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(higher_order.neg());
        Ok(())
    })
}

/// Removes explicitly stored zero contributions.
///
/// # Safety
/// `higher_order` must be a valid mutable higher-order handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_clean(higher_order: *mut LmHigherOrder) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { mut_from_c(higher_order as *mut HigherOrder, "higher_order") }?;
        higher_order.clean();
        Ok(())
    })
}

/// Sets the bias for a specific entry (variable tuple) of the higher-order term.
///
/// # Safety
/// `higher_order` must be a valid mutable higher-order handle created by this library.
/// `indices` must point to `indices_len` readable `uint32_t`s (may be null iff `indices_len == 0`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_set_bias(
    higher_order: *mut LmHigherOrder,
    indices: *const u32,
    indices_len: usize,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { mut_from_c(higher_order as *mut HigherOrder, "higher_order") }?;

        let indices = unsafe { opt_slice_from_c(indices, indices_len) }.unwrap_or(&[]);

        higher_order[indices] = bias;
        Ok(())
    })
}

/// Sets the bias stored under a canonical higher-order key.
///
/// # Safety
/// `higher_order` and `key` must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_set_bias_key(
    higher_order: *mut LmHigherOrder,
    key: *const c_char,
    bias: f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { mut_from_c(higher_order as *mut HigherOrder, "higher_order") }?;
        let key = unsafe { str_from_c(key, "key") }?.to_owned();
        higher_order[&key] = bias;
        Ok(())
    })
}
