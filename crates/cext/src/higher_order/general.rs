use crate::utils::{handle_to_c, out_from_c, ref_from_c};
use luna_model::core::prelude::HigherOrder;

use crate::{LmHigherOrder, LmStatus, error::guard};

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
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let other = unsafe { ref_from_c(other as *const HigherOrder, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = higher_order.eq(other);
        Ok(())
    })
}

/// Clones a higher_order term into a new, independent handle written to `out`.
///
/// # Safety
/// `higher_order` must be a valid higher-order handle created by this library.
/// `out` must point to writable `LmHigherOrder *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_clone(
    higher_order: *const LmHigherOrder,
    out: *mut *mut LmHigherOrder,
) -> LmStatus {
    guard(|| {
        let higher_order =
            unsafe { ref_from_c(higher_order as *const HigherOrder, "higher_order") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(higher_order.clone());
        Ok(())
    })
}
