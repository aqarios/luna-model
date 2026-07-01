use luna_model::core::prelude::HigherOrder;

use crate::{
    LmHigherOrder, LmStatus,
    error::{CapiError, guard},
};

/// Compares two higher_order terms for equality.
///
/// # Safety
/// `higher_order` must be a valid higher_order handle created by this library.
/// `other` must be a valid higher_order handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_eq(
    higher_order: *const LmHigherOrder,
    other: *const LmHigherOrder,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let other = unsafe { (other as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = higher_order.eq(other);
        Ok(())
    })
}

/// Clones a higher_order term into a new, independent handle written to `out`.
///
/// # Safety
/// `higher_order` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_clone(
    higher_order: *const LmHigherOrder,
    out: *mut *mut LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let higher_order = unsafe { (higher_order as *const HigherOrder).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("higher_order"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(higher_order.clone())) as *mut LmHigherOrder;
        Ok(())
    })
}
