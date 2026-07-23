use lunamodel_core::prelude::HigherOrder;

use crate::{
    LmHigherOrder, LmHigherOrderItem, LmHigherOrderItemBuffer, LmStatus, LmStrBuffer,
    buffer::{higher_order_item_buffer_from_vec, str_buffer_from_vec, u32_buffer_from_vec},
    error::guard,
    utils::{opt_slice_from_c, out_from_c, ref_from_c, str_from_c},
};
use std::ffi::c_char;

/// Returns the number of explicitly stored entries of the higher_order term.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_len(
    higher_order: *const LmHigherOrder,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = higher_order.len();
        Ok(())
    })
}

/// Returns whether the higher_order term is zero.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_is_zero(
    higher_order: *const LmHigherOrder,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = higher_order.is_zero();
        Ok(())
    })
}

/// Returns whether the higher_order term is empty.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_is_empty(
    higher_order: *const LmHigherOrder,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = higher_order.is_empty();
        Ok(())
    })
}

/// Returns the degree of the higher_order term.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_degree(
    higher_order: *const LmHigherOrder,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = higher_order.degree();
        Ok(())
    })
}

/// Returns the higher_order term items.
///
/// The returned array is owned by the caller and must be released with [`lm_higher_order_item_buffer_free`].
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `LmHigherOrderItemBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_items(
    higher_order: *const LmHigherOrder,
    out: *mut LmHigherOrderItemBuffer,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        let items = higher_order
            .iter_contrib()
            .map(|(indices, bias)| LmHigherOrderItem {
                indices: u32_buffer_from_vec(indices),
                bias,
            })
            .collect::<Vec<_>>();
        *out = higher_order_item_buffer_from_vec(items);
        Ok(())
    })
}

/// Returns the canonical keys stored by the higher-order term.
///
/// # Safety
/// `higher_order` must be valid and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_keys(
    higher_order: *const LmHigherOrder,
    out: *mut LmStrBuffer,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        *unsafe { out_from_c(out) }? =
            str_buffer_from_vec(higher_order.iter().map(|(key, _)| key.clone()).collect());
        Ok(())
    })
}

/// Returns the bias stored in the higher_order term for the given variable index.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `double` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_index(
    higher_order: *const LmHigherOrder,
    indices: *const u32,
    indices_len: usize,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;

        let indices = unsafe { opt_slice_from_c(indices, indices_len) }.unwrap_or(&[]);

        *out = higher_order[indices];
        Ok(())
    })
}

/// Returns the bias stored under a canonical higher-order key.
///
/// # Safety
/// `higher_order` and `key` must be valid and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_index_key(
    higher_order: *const LmHigherOrder,
    key: *const c_char,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let key = unsafe { str_from_c(key, "key") }?.to_owned();
        *unsafe { out_from_c(out) }? = higher_order[&key];
        Ok(())
    })
}
