use luna_model::core::{Solution, prelude::ContentEquality};

use crate::{
    LmModel, LmSol, LmStatus,
    error::{CapiError, guard},
    utils::{new_handle_out_from_c, opt_from_c, out_from_c, shared_from_c, slice_from_c},
};

/// Clones a solution into a new, independent handle written to `out`.
///
/// # Safety
/// `sol` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_clone(sol: *const LmSol, out: *mut *mut LmSol) -> LmStatus {
    guard(|| {
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        unsafe { new_handle_out_from_c(out, sol.clone()) }
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
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let other = unsafe { shared_from_c(other, "other") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.eq(&other);
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
        let sol = unsafe { shared_from_c(sol, "sol") }?.read_arc();
        let other = unsafe { shared_from_c(other, "other") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sol.equal_contents(&other);
        Ok(())
    })
}

/// Merges multiple solutions that share the same variable schema and sense.
///
/// The merge is performed by appending the column-oriented storage for each
/// participating solution and then aggregating duplicate rows. If `model` is
/// provided, the merged solution is re-evaluated afterwards so derived fields
/// such as objective values and feasibility reflect the final merged state.
/// `tol` is only used during that optional re-evaluation, where it controls
/// floating-point constraint comparisons.
///
/// # Safety
/// `sols` must be a valid solution handle buffer created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sol_merge_many(
    sols: *const *const LmSol,
    sols_len: usize,
    model: *const LmModel,
    tol: *const f64,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let ptrs = unsafe { slice_from_c(sols, sols_len, "sols") }?;
        let solutions = ptrs
            .iter()
            .map(|&p| unsafe { shared_from_c(p, "sol") }.map(|sol| sol.read_arc().clone()))
            .collect::<Result<Vec<Solution>, _>>()?;
        let model = if model.is_null() {
            None
        } else {
            Some(unsafe { shared_from_c(model, "model") }?.read_arc())
        };
        let tol = unsafe { opt_from_c(tol) };
        let merged =
            Solution::merge_many(&solutions, model.as_deref(), tol).map_err(CapiError::errored)?;
        unsafe { new_handle_out_from_c(out, merged) }
    })
}
