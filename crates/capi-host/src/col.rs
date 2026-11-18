use lunamodel_core::solution::Column;

use crate::{
    LmAssignment, LmColumn, LmStatus, LmVtype,
    error::{CapiError, guard},
    utils::{
        assignment_from_c, assignment_to_c, drop_from_c, handle_to_c, mut_from_c, opt_from_c,
        out_from_c, ref_from_c, slice_from_c, vtype_to_c,
    },
};

/// Frees a column handle. Passing null is a no-op.
///
/// # Safety
/// `column` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_free(column: *mut LmColumn) {
    unsafe { drop_from_c(column as *mut Column) }
}

/// Creates a column from raw numeric values and a logical variable type.
///
/// # Safety
/// `data` must reference `len` readable doubles and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_new(
    vtype: LmVtype,
    data: *const f64,
    len: usize,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let data = unsafe { slice_from_c(data, len, "data") }?.to_vec();
        let column = match vtype {
            LmVtype::Binary => Column::binary(data),
            LmVtype::Spin => Column::spin(data),
            LmVtype::Integer => Column::integer(data),
            LmVtype::Real => Column::real(data),
            LmVtype::InvertedBinary => {
                return Err(CapiError::new(
                    LmStatus::InvalidArgument,
                    "inverted binary has no standalone column",
                ));
            }
        };
        *unsafe { out_from_c(out) }? = handle_to_c(column);
        Ok(())
    })
}

/// Creates a one-row column from a typed assignment.
///
/// # Safety
/// `out` must point to writable `LmColumn *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_with(
    assignment: LmAssignment,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let assignment = assignment_from_c(assignment)?;
        *unsafe { out_from_c(out) }? = handle_to_c(Column::with(assignment));
        Ok(())
    })
}

/// Clones a column.
///
/// # Safety
/// `column` must be valid and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_clone(
    column: *const LmColumn,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        *unsafe { out_from_c(out) }? = handle_to_c(column.clone());
        Ok(())
    })
}

/// Compares two columns structurally.
///
/// # Safety
/// Both columns must be valid and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_eq(
    column: *const LmColumn,
    other: *const LmColumn,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let other = unsafe { ref_from_c(other as *const Column, "other") }?;
        *unsafe { out_from_c(out) }? = column == other;
        Ok(())
    })
}

/// Returns a filtered copy of a column.
///
/// # Safety
/// `column` and `mask` must be readable and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_filter_by_mask(
    column: *const LmColumn,
    mask: *const bool,
    len: usize,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let mask = unsafe { slice_from_c(mask, len, "mask") }?;
        *unsafe { out_from_c(out) }? = handle_to_c(column.filter_by_mask(mask));
        Ok(())
    })
}

/// Extracts one row into a new one-row column.
///
/// # Safety
/// `column` must be valid and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_extract(
    column: *const LmColumn,
    row: usize,
    out: *mut *mut LmColumn,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        if row >= column.len() {
            return Err(CapiError::new(
                LmStatus::Absent,
                format!("column has no value at index '{row}'"),
            ));
        }
        *unsafe { out_from_c(out) }? = handle_to_c(column.extract(row));
        Ok(())
    })
}

/// Returns the logical variable type stored by the column.
///
/// # Safety
/// `column` must be a valid column handle and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_vtype(column: *const LmColumn, out: *mut LmVtype) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = vtype_to_c(match column {
            Column::Binary(_) => lunamodel_types::Vtype::Binary,
            Column::Spin(_) => lunamodel_types::Vtype::Spin,
            Column::Integer(_) => lunamodel_types::Vtype::Integer,
            Column::Real(_) => lunamodel_types::Vtype::Real,
        });
        Ok(())
    })
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

/// Returns a borrowed view of the raw numeric values.
///
/// The pointer remains valid until the column is mutated or freed.
///
/// # Safety
/// `column`, `data`, and `len` must be valid pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_values(
    column: *const LmColumn,
    data: *mut *const f64,
    len: *mut usize,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        let values = match column {
            Column::Binary(values) => &values.0,
            Column::Spin(values) => &values.0,
            Column::Integer(values) => &values.0,
            Column::Real(values) => &values.0,
        };
        *unsafe { out_from_c(data) }? = values.as_ptr();
        *unsafe { out_from_c(len) }? = values.len();
        Ok(())
    })
}

/// Appends a typed assignment.
///
/// # Safety
/// `column` must be a valid mutable handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_push(column: *mut LmColumn, assignment: LmAssignment) -> LmStatus {
    guard(|| {
        let column = unsafe { mut_from_c(column as *mut Column, "column") }?;
        column
            .push(assignment_from_c(assignment)?)
            .map_err(CapiError::from)?;
        Ok(())
    })
}

/// Attempts to append a numeric value using the column's native coercion.
///
/// # Safety
/// `column` must be a valid mutable handle and `tol` may be null or readable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_try_push(
    column: *mut LmColumn,
    value: f64,
    tol: *const f64,
) -> LmStatus {
    guard(|| {
        let column = unsafe { mut_from_c(column as *mut Column, "column") }?;
        column
            .try_push(value, unsafe { opt_from_c(tol) })
            .map_err(CapiError::from)?;
        Ok(())
    })
}

/// Removes one row from the column.
///
/// # Safety
/// `column` must be a valid mutable handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_remove(column: *mut LmColumn, index: usize) -> LmStatus {
    guard(|| {
        let column = unsafe { mut_from_c(column as *mut Column, "column") }?;
        if index >= column.len() {
            return Err(CapiError::new(
                LmStatus::Absent,
                format!("column has no value at index '{index}'"),
            ));
        }
        column.remove(index);
        Ok(())
    })
}

/// Returns the element at the index.
///
/// The pointer remains valid until the column is mutated or freed.
///
/// # Safety
/// `column` and `value` must be valid pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_col_index(
    column: *const LmColumn,
    index: usize,
    value: *mut f64,
) -> LmStatus {
    guard(|| {
        let column = unsafe { ref_from_c(column as *const Column, "column") }?;
        *unsafe { out_from_c(value) }? = column[index];
        Ok(())
    })
}
