use luna_model::core::prelude::VarRef;
use lunamodel_cext_handle::{ConstraintCollectionHandle, new_shared};

use crate::{
    LmCC, LmExpr, LmModel, LmResultView, LmSampleView, LmSol, LmStatus, LmVref,
    error::{CapiError, guard},
    res::ResultViewHandle,
    sample::SampleViewHandle,
    utils::{
        handle_to_c, new_handle_out_from_c, opt_from_c, out_from_c, ref_from_c, shared_from_c,
        storage_from_c, storage_out_from_c,
    },
};

/// Evaluates the model for a given solution.
///
/// On success, `out` receives an owned handle to the evaluated solution that
/// must be released with [`lm_sol_free`].
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `solution` must be a valid solution handle created by this library.
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_evaluate_solution(
    model: *const LmModel,
    solution: *const LmSol,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let sol = unsafe { shared_from_c(solution, "solution") }?.read_arc();
        let res = model.evaluate_solution(&sol).map_err(CapiError::errored)?;
        unsafe { new_handle_out_from_c(out, res) }
    })
}

/// Evaluates the model for a given solution with a feasibility tolerance.
///
/// `tol == NULL` means use the default tolerance.
///
/// On success, `out` receives an owned handle to the evaluated solution that
/// must be released with [`lm_sol_free`].
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `solution` must be a valid solution handle created by this library.
/// `tol` must be null or point to a readable `double`.
/// `out` must point to writable `LmSol *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_evaluate_solution_with_tol(
    model: *const LmModel,
    solution: *const LmSol,
    tol: *const f64,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let sol = unsafe { shared_from_c(solution, "solution") }?.read_arc();
        let tol = unsafe { opt_from_c(tol) };

        let res = model
            .evaluate_solution_with_tol(&sol, tol)
            .map_err(CapiError::errored)?;
        unsafe { new_handle_out_from_c(out, res) }
    })
}

/// Evaluates the model against a single sample row.
///
/// The returned result view owns a synchronized one-row evaluated solution and
/// remains valid independently of the input sample handle.
///
/// # Safety
/// `model` and `sample` must be valid handles. `tol` must be null or point to a
/// readable `double`. `out` must point to writable `LmResultView *` storage; the
/// returned handle must be released with [`crate::res::lm_result_view_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_evaluate_sample(
    model: *const LmModel,
    sample: *const LmSampleView,
    tol: *const f64,
    out: *mut *mut LmResultView,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let sample = unsafe { ref_from_c(sample as *const SampleViewHandle, "sample") }?;
        let row = {
            let solution = sample.sol.read();
            if sample.index >= solution.len() {
                return Err(CapiError::new(
                    LmStatus::Absent,
                    "sample index is out of range",
                ));
            }
            solution.extract(sample.index)
        };
        let evaluated = model
            .evaluate_solution_with_tol(&row, unsafe { opt_from_c(tol) })
            .map_err(CapiError::errored)?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(ResultViewHandle {
            sol: new_shared::<LmSol>(evaluated),
            index: 0,
        });
        Ok(())
    })
}

/// Substitutes every occurrence of `target` in the model with `replacement`.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `target` must be a valid vref handle created by this library.
/// `replacement` must be a valid expression handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_substitute(
    model: *mut LmModel,
    target: *const LmVref,
    replacement: *const LmExpr,
) -> LmStatus {
    guard(|| {
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { storage_from_c(replacement, "replacement") }?
            .read()
            .clone();
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
        model
            .substitute(target, &replacement)
            .map_err(CapiError::errored)?;
        Ok(())
    })
}

/// Returns the subset of model constraints violated by a sample view.
///
/// # Safety
/// `model` and `sample` must be valid handles, `tol` may be null or readable,
/// and `out` must point to writable `LmCC *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_violated_constraints(
    model: *const LmModel,
    sample: *const LmSampleView,
    tol: *const f64,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let sample = unsafe { ref_from_c(sample as *const SampleViewHandle, "sample") }?;
        let solution = sample.sol.read();
        let sample = solution
            .sample(sample.index)
            .ok_or_else(|| CapiError::new(LmStatus::Absent, "sample index is out of range"))?;
        let tol = unsafe { opt_from_c(tol) };
        let constraints = model
            .violated_constraints(&sample, tol)
            .map_err(CapiError::errored)?;
        unsafe { storage_out_from_c(out, ConstraintCollectionHandle::new(constraints)) }
    })
}
