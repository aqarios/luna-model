use luna_model::core::prelude::HigherOrder;

use crate::{
    LmHigherOrder, LmHigherOrderItem, LmHigherOrderItemBuffer, LmStatus,
    buffer::{higher_order_item_buffer_from_vec, u32_buffer_from_vec},
    error::{CapiError, guard},
};

/// Returns the len of the higher_order term.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_len(
    higher_order: *const LmHigherOrder,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = higher_order.len();
        Ok(())
    })
}

/// Returns if the higher_order term is zero.
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
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = higher_order.is_zero();
        Ok(())
    })
}

/// Returns if the higher_order term is empty.
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
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = higher_order.is_empty();
        Ok(())
    })
}

/// Returns the degree of the higher_order term.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_degree(
    higher_order: *const LmHigherOrder,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
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

/// Returns the bias in the higher_order term for the var idx.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `out` must point to writable `f64` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_index(
    higher_order: *const LmHigherOrder,
    indices: *const u32,
    indices_len: usize,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        // `from_raw_parts` requires a non-null pointer, so treat empty as no indices.
        let indices: &[u32] = if indices.is_null() || indices_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(indices, indices_len) }
        };

        *out = higher_order[indices];
        Ok(())
    })
}
