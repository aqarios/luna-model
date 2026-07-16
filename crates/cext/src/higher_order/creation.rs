use luna_model::core::prelude::HigherOrder;

use crate::{
    LmHigherOrder,
    utils::{drop_from_c, handle_to_c},
};

/// Frees a higher_order handle. Passing null is a no-op.
///
/// # Safety
/// `higher_order` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_higher_order_free(higher_order: *mut LmHigherOrder) {
    unsafe { drop_from_c(higher_order as *mut HigherOrder) }
}

/// Creates a new empty higher_order term.
///
/// Returns a non-null handle that must be released with [`lm_higher_order_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_higher_order_default() -> *mut LmHigherOrder {
    handle_to_c(HigherOrder::default())
}

/// Creates a new higher_order term with some capacity.
///
/// Returns a non-null handle that must be released with [`lm_higher_order_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_higher_order_with_capacity(cap: usize) -> *mut LmHigherOrder {
    handle_to_c(HigherOrder::with_capacity(cap))
}
