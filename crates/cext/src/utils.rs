use std::ffi::{CStr, c_char};

use luna_model::{
    core::{
        ValueSource,
        solution::{Assignment, result::ResultView},
    },
    types::{Bound, Comparator, Ctype, Sense, Vtype},
};

use crate::{
    LmAssignment, LmBound, LmBoundTag, LmComparator, LmCtype, LmResultView, LmSense, LmStatus,
    LmValueSource, LmVtype, error::CapiError, res::ResultViewHandle,
};

pub fn sense_from_c(sense: LmSense) -> Sense {
    match sense {
        LmSense::Min => Sense::Min,
        LmSense::Max => Sense::Max,
    }
}

pub fn sense_to_c(sense: Sense) -> LmSense {
    match sense {
        Sense::Min => LmSense::Min,
        Sense::Max => LmSense::Max,
    }
}

pub fn vtype_to_c(vtype: Vtype) -> LmVtype {
    match vtype {
        Vtype::Binary => LmVtype::Binary,
        Vtype::InvertedBinary => LmVtype::InvertedBinary,
        Vtype::Spin => LmVtype::Spin,
        Vtype::Integer => LmVtype::Integer,
        Vtype::Real => LmVtype::Real,
    }
}

pub fn vtype_from_c(vtype: LmVtype) -> Vtype {
    match vtype {
        LmVtype::Binary => Vtype::Binary,
        LmVtype::InvertedBinary => Vtype::InvertedBinary,
        LmVtype::Spin => Vtype::Spin,
        LmVtype::Integer => Vtype::Integer,
        LmVtype::Real => Vtype::Real,
    }
}

pub fn ctype_to_c(ctype: Ctype) -> LmCtype {
    match ctype {
        Ctype::Unconstrained => LmCtype::Unconstrained,
        Ctype::Equality => LmCtype::Equality,
        Ctype::Inequality => LmCtype::Inequality,
        Ctype::LessEqual => LmCtype::LessEqual,
        Ctype::GreaterEqual => LmCtype::GreaterEqual,
    }
}

pub fn ctype_from_c(ctype: LmCtype) -> Ctype {
    match ctype {
        LmCtype::Unconstrained => Ctype::Unconstrained,
        LmCtype::Equality => Ctype::Equality,
        LmCtype::Inequality => Ctype::Inequality,
        LmCtype::LessEqual => Ctype::LessEqual,
        LmCtype::GreaterEqual => Ctype::GreaterEqual,
    }
}

pub fn cmp_to_c(cmp: Comparator) -> LmComparator {
    match cmp {
        Comparator::Eq => LmComparator::Eq,
        Comparator::Le => LmComparator::Le,
        Comparator::Ge => LmComparator::Ge,
    }
}

pub fn cmp_from_c(cmp: LmComparator) -> Comparator {
    match cmp {
        LmComparator::Eq => Comparator::Eq,
        LmComparator::Le => Comparator::Le,
        LmComparator::Ge => Comparator::Ge,
    }
}

pub fn bound_from_c(bound: LmBound) -> Bound {
    match bound.tag {
        LmBoundTag::Unbounded => Bound::Unbounded,
        LmBoundTag::Bounded => Bound::Bounded(bound.value),
    }
}

pub fn bound_to_c(bound: Bound) -> LmBound {
    match bound {
        Bound::Unbounded => LmBound {
            tag: LmBoundTag::Unbounded,
            value: 0.0,
        },
        Bound::Bounded(val) => LmBound {
            tag: LmBoundTag::Bounded,
            value: val,
        },
    }
}

pub fn value_source_from_c(vs: LmValueSource) -> ValueSource {
    match vs {
        LmValueSource::Raw => ValueSource::Raw,
        LmValueSource::Obj => ValueSource::Obj,
    }
}

// ///////////////////////////// //
// Pointer / FFI value mappings  //
// ///////////////////////////// //

/// Borrows the Rust value behind a C pointer, failing with `NullPointer`.
///
/// # Safety
/// `ptr` must be null or valid for reads as a `T` for the returned lifetime.
pub(crate) unsafe fn ref_from_c<'a, T>(ptr: *const T, what: &str) -> Result<&'a T, CapiError> {
    unsafe { ptr.as_ref() }.ok_or_else(|| CapiError::null_pointer(what))
}

/// Borrows the Rust value behind a C pointer, returning None with `NullPointer`.
///
/// # Safety
/// `ptr` must be null or valid for reads as a `T` for the returned lifetime.
pub(crate) unsafe fn opt_ref_from_c<'a, T>(ptr: *const T) -> Option<&'a T> {
    unsafe { ptr.as_ref() }
}

/// Mutably borrows the Rust value behind a C pointer, failing with `NullPointer`.
///
/// # Safety
/// `ptr` must be null or valid for reads and writes as a `T` for the returned
/// lifetime, without aliasing.
pub(crate) unsafe fn mut_from_c<'a, T>(ptr: *mut T, what: &str) -> Result<&'a mut T, CapiError> {
    unsafe { ptr.as_mut() }.ok_or_else(|| CapiError::null_pointer(what))
}

/// Dereferences an `out` parameter, failing with `NullPointer`.
///
/// # Safety
/// `out` must be null or point to writable `T` storage.
pub(crate) unsafe fn out_from_c<'a, T>(out: *mut T) -> Result<&'a mut T, CapiError> {
    unsafe { mut_from_c(out, "out") }
}

/// Moves a Rust value to the heap and returns it as an owned C handle.
pub(crate) fn handle_to_c<T, C>(value: T) -> *mut C {
    Box::into_raw(Box::new(value)) as *mut C
}

/// Reclaims and drops a Rust value previously leaked as a C handle.
///
/// Passing null is a no-op.
///
/// # Safety
/// `ptr` must be null or originate from [`handle_to_c`] (or an equivalent
/// `Box::into_raw`) for `T` and must not be used or freed again.
pub(crate) unsafe fn drop_from_c<T>(ptr: *mut T) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) });
    }
}

/// Borrows a required NUL-terminated UTF-8 C string.
///
/// Fails with `NullPointer` when null and `InvalidArgument` when not UTF-8.
///
/// # Safety
/// `ptr` must be null or point to a valid NUL-terminated C string.
pub(crate) unsafe fn str_from_c<'a>(ptr: *const c_char, what: &str) -> Result<&'a str, CapiError> {
    if ptr.is_null() {
        return Err(CapiError::null_pointer(what));
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|e| {
        CapiError::new(
            LmStatus::InvalidArgument,
            format!("`{what}` must be valid UTF-8: {e}"),
        )
    })
}

/// Borrows an optional NUL-terminated UTF-8 C string (null means `None`).
///
/// Fails with `InvalidArgument` when non-null but not UTF-8.
///
/// # Safety
/// `ptr` must be null or point to a valid NUL-terminated C string.
pub(crate) unsafe fn opt_str_from_c<'a>(
    ptr: *const c_char,
    what: &str,
) -> Result<Option<&'a str>, CapiError> {
    if ptr.is_null() {
        return Ok(None);
    }
    unsafe { str_from_c(ptr, what) }.map(Some)
}

/// Borrows a required `(ptr, len)` C array, failing with `NullPointer`.
///
/// # Safety
/// `data` must be null or point to `len` readable `T` values.
pub(crate) unsafe fn slice_from_c<'a, T>(
    data: *const T,
    len: usize,
    what: &str,
) -> Result<&'a [T], CapiError> {
    if data.is_null() {
        return Err(CapiError::null_pointer(what));
    }
    Ok(unsafe { std::slice::from_raw_parts(data, len) })
}

/// Borrows an optional `(ptr, len)` C array.
///
/// Null means `None`; non-null with `len == 0` is an explicit empty slice.
///
/// # Safety
/// `data` must be null or point to `len` readable `T` values.
pub(crate) unsafe fn opt_slice_from_c<'a, T>(data: *const T, len: usize) -> Option<&'a [T]> {
    if data.is_null() {
        None
    } else if len == 0 {
        // `from_raw_parts` is only defined for non-null, well-aligned data.
        Some(&[])
    } else {
        Some(unsafe { std::slice::from_raw_parts(data, len) })
    }
}

/// Copies an optional scalar from a nullable C pointer (null means `None`).
///
/// # Safety
/// `ptr` must be null or valid for reads as a `T`.
pub(crate) unsafe fn opt_from_c<T: Copy>(ptr: *const T) -> Option<T> {
    unsafe { ptr.as_ref() }.copied()
}

pub fn assignment_to_c(assignment: Assignment) -> LmAssignment {
    match assignment {
        Assignment::Binary(v) => LmAssignment {
            vtype: LmVtype::Binary,
            value: v as f64,
        },
        Assignment::Spin(v) => LmAssignment {
            vtype: LmVtype::Spin,
            value: v as f64,
        },
        Assignment::Integer(v) => LmAssignment {
            vtype: LmVtype::Integer,
            value: v as f64,
        },
        Assignment::Real(v) => LmAssignment {
            vtype: LmVtype::Real,
            value: v,
        },
    }
}

/// Rebuilds the borrowed `ResultView` backing an `LmResultView` handle.
///
/// # Safety
/// `view` must be a valid result-view handle created by this library whose
/// originating solution is still alive.
pub unsafe fn result_view_from_c<'a>(
    view: *const LmResultView,
) -> Result<ResultView<'a>, CapiError> {
    let handle = unsafe { ref_from_c(view as *const ResultViewHandle, "view") }?;
    let sol = unsafe { ref_from_c(handle.sol, "sol") }?;
    Ok(ResultView::new(sol, handle.index))
}
