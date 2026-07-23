use crate::{
    LmBound, LmStatus,
    error::guard,
    utils::bound_from_c,
    utils::{out_from_c, ref_from_c},
};

/// Returns whether the bound is finite (`Bounded`) rather than `Unbounded`.
///
/// # Safety
/// `bound` must point to a readable `LmBound`.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bound_is_bounded(bound: *const LmBound, out: *mut bool) -> LmStatus {
    guard(|| {
        let bound = bound_from_c(*unsafe { ref_from_c(bound, "bound") }?);
        let out = unsafe { out_from_c(out) }?;
        *out = bound.is_bounded();
        Ok(())
    })
}

/// Returns the finite bound value, or `Absent` for an unbounded endpoint.
///
/// # Safety
/// `bound` must be readable and `out` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bound_value(bound: *const LmBound, out: *mut f64) -> LmStatus {
    guard(|| {
        let bound = bound_from_c(*unsafe { ref_from_c(bound, "bound") }?);
        let out = unsafe { out_from_c(out) }?;
        match bound {
            lunamodel_types::Bound::Bounded(value) => {
                *out = value;
                Ok(())
            }
            lunamodel_types::Bound::Unbounded => Err(crate::error::CapiError::new(
                LmStatus::Absent,
                "bound is unbounded",
            )),
        }
    })
}

/// Compares two bound endpoints.
///
/// # Safety
/// `bound`, `other`, and `out` must be valid readable/writable pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bound_eq(
    bound: *const LmBound,
    other: *const LmBound,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let bound = bound_from_c(*unsafe { ref_from_c(bound, "bound") }?);
        let other = bound_from_c(*unsafe { ref_from_c(other, "other") }?);
        let out = unsafe { out_from_c(out) }?;
        *out = bound == other;
        Ok(())
    })
}
