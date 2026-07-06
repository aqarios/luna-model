use luna_model::core::solution::Column;

use crate::{LmColumn, utils::drop_from_c};

/// Frees a column handle. Passing null is a no-op.
///
/// # Safety
/// `column` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_free(column: *mut LmColumn) {
    unsafe { drop_from_c(column as *mut Column) }
}
