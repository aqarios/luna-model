use luna_model::core::{Solution, prelude::ContentEquality};

use crate::{
    LmSol, LmStatus,
    error::guard,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Clones a solution into a new, independent handle written to `out`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_clone(sol: *const LmSol, out: *mut *mut LmSol) -> LmStatus {
    guard(|| {
        let sol = unsafe { ref_from_c(sol as *const Solution, "sol") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(sol.clone());
        Ok(())
    })
}

/// Compares two solutions for equality.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `other` must be a valid solution handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_eq(
    sol: *const LmSol,
    other: *const LmSol,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { ref_from_c(sol as *const Solution, "sol") }?;
        let other = unsafe { ref_from_c(other as *const Solution, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = sol.eq(other);
        Ok(())
    })
}

/// Compares two solutionss for equal contents.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `other` must be a valid solution handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_equal_contents(
    sol: *const LmSol,
    other: *const LmSol,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let sol = unsafe { ref_from_c(sol as *const Solution, "sol") }?;
        let other = unsafe { ref_from_c(other as *const Solution, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = sol.equal_contents(other);
        Ok(())
    })
}
