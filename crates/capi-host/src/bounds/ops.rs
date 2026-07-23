use lunamodel_core::Bounds;

use crate::{
    error::CapiError,
    utils::{opt_from_c, out_from_c, ref_from_c},
};

use crate::{LmBounds, LmStatus, error::guard};

/// Returns whether `val` lies within the closed interval represented by these bounds.
/// Unbounded sides are treated as always satisfied.
///
/// # Safety
/// `bounds` must be a valid bounds handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_eval(
    bounds: *const LmBounds,
    value: f64,
    tol: *const f64,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        let tol = unsafe { opt_from_c(tol) };
        *out = (bounds.evaluate(value, tol)).map_err(CapiError::errored)?;
        Ok(())
    })
}
