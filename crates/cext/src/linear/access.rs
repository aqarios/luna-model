use luna_model::core::prelude::Linear;

use crate::{
    LmLinear, LmLinearItem, LmLinearItemBuffer, LmStatus,
    buffer::linear_item_buffer_from_vec,
    error::{CapiError, guard},
};

/// Returns the len of the linear term.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `usize` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_len(linear: *const LmLinear, out: *mut usize) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = linear.len();
        Ok(())
    })
}

/// Returns if the linear term is zero.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_is_zero(linear: *const LmLinear, out: *mut bool) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = linear.is_zero();
        Ok(())
    })
}

/// Returns if the linear term is empty.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_is_empty(linear: *const LmLinear, out: *mut bool) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = linear.is_empty();
        Ok(())
    })
}

/// Returns the linear term items.
///
/// The returned array is owned by the caller and must be released with [`lm_linear_item_buffer_free`].
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `LmLinearItemBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_items(
    linear: *const LmLinear,
    out: *mut LmLinearItemBuffer,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let items = linear
            .iter()
            .map(|(idx, bias)| LmLinearItem { idx, bias })
            .collect::<Vec<_>>();
        *out = linear_item_buffer_from_vec(items);
        Ok(())
    })
}

/// Returns the bias in the linear term for the var idx.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `out` must point to writable `f64` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_index(
    linear: *const LmLinear,
    index: u32,
    out: *mut f64,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = linear[index];
        Ok(())
    })
}
