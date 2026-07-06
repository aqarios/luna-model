use luna_model::core::solution::Column;

use crate::{
    LmAssignment, LmColumn, LmStatus,
    error::{CapiError, guard},
    utils::{assignment_to_c, drop_from_c, out_from_c, ref_from_c},
};

/// Frees a column handle. Passing null is a no-op.
///
/// # Safety
/// `column` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_free(column: *mut LmColumn) {
    unsafe { drop_from_c(column as *mut Column) }
}

/// Returns the number of stored values (sample rows) in the column.
///
/// # Safety
/// `column` must be a valid column handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_len(column: *const LmColumn, out: *mut usize) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = column.len();
        Ok(())
    })
}

/// Returns whether the column has no stored values.
///
/// # Safety
/// `column` must be a valid column handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_is_empty(column: *const LmColumn, out: *mut bool) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = column.is_empty();
        Ok(())
    })
}

/// Returns the typed assignment at a row position in the column.
///
/// # Safety
/// `column` must be a valid column handle created by this library.
/// `out` must point to writable `LmAssignment` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_assignment(
    column: *const LmColumn,
    index: usize,
    out: *mut LmAssignment,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let out = unsafe { out_from_c(out) }?;
        if index >= column.len() {
            return Err(CapiError::new(
                LmStatus::Absent,
                format!("column has no value at index '{index}'"),
            ));
        }
        *out = assignment_to_c(column.as_assignment(index));
        Ok(())
    })
}
