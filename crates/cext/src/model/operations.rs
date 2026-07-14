use crate::utils::{handle_to_c, mut_from_c, opt_from_c, out_from_c, ref_from_c};
use luna_model::core::solution::sample::SampleView;
use luna_model::core::{Expression, Model, Solution, prelude::VarRef};

use crate::res::ResultViewHandle;
use crate::sample::SampleViewHandle;
use crate::{
    LmCC, LmExpr, LmModel, LmResultView, LmSampleView, LmSol, LmStatus, LmVref,
    error::{CapiError, guard},
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
        let model = unsafe { ref_from_c(model as *const Model, "model") }?;
        let sol = unsafe { ref_from_c(solution as *const Solution, "solution") }?;
        let out = unsafe { out_from_c(out) }?;
        let res = model.evaluate_solution(sol).map_err(CapiError::errored)?;
        *out = handle_to_c(res);
        Ok(())
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
        let model = unsafe { ref_from_c(model as *const Model, "model") }?;
        let sol = unsafe { ref_from_c(solution as *const Solution, "solution") }?;
        let tol = unsafe { opt_from_c(tol) };
        let out = unsafe { out_from_c(out) }?;

        let res = model
            .evaluate_solution_with_tol(sol, tol)
            .map_err(CapiError::errored)?;
        *out = handle_to_c(res);
        Ok(())
    })
}

/// Evaluates the model against a single sample row, returning the enriched
/// result view for that row.
///
/// The sample is extracted into a one-row solution and evaluated, mirroring the
/// core evaluation path. `tol == NULL` uses the default tolerance.
///
/// # Safety
/// `model` and `sample` must be valid handles. `tol` must be null or point to a
/// readable `double`. `out` must point to writable `LmResultView *` storage; the
/// returned handle must be released with [`lm_result_view_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_evaluate_sample(
    model: *const LmModel,
    sample: *const LmSampleView,
    tol: *const f64,
    out: *mut *mut LmResultView,
) -> LmStatus {
    guard(|| {
        let model = unsafe { ref_from_c(model as *const Model, "model") }?;
        let sample = unsafe { ref_from_c(sample as *const SampleViewHandle, "sample") }?;
        let tol = unsafe { opt_from_c(tol) };
        let out = unsafe { out_from_c(out) }?;

        let row = sample.sol.extract(sample.index);
        let evaluated = model
            .evaluate_solution_with_tol(&row, tol)
            .map_err(CapiError::errored)?;
        *out = handle_to_c(ResultViewHandle {
            sol: evaluated,
            index: 0,
        });
        Ok(())
    })
}

/// Returns the constraints violated by a single sample row.
///
/// `tol == NULL` uses the default tolerance for the floating-point comparisons.
///
/// # Safety
/// `model` and `sample` must be valid handles. `tol` must be null or point to a
/// readable `double`. `out` must point to writable `LmCC *` storage; the returned
/// handle must be released with [`lm_cc_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_violated_constraints(
    model: *const LmModel,
    sample: *const LmSampleView,
    tol: *const f64,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let model = unsafe { ref_from_c(model as *const Model, "model") }?;
        let sample = unsafe { ref_from_c(sample as *const SampleViewHandle, "sample") }?;
        let tol = unsafe { opt_from_c(tol) };
        let out = unsafe { out_from_c(out) }?;

        let view = SampleView::new(&sample.sol, sample.index);
        let violated = model
            .violated_constraints(&view, tol)
            .map_err(CapiError::errored)?;
        *out = handle_to_c(violated);
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
        let model = unsafe { mut_from_c(model as *mut Model, "model") }?;
        let target = unsafe { ref_from_c(target as *const VarRef, "target") }?;
        let replacement = unsafe { ref_from_c(replacement as *const Expression, "replacement") }?;

        model
            .substitute(target, replacement)
            .map_err(CapiError::errored)?;

        Ok(())
    })
}
