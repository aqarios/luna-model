use luna_model::core::{Expression, Model, Solution, prelude::VarRef};

use crate::{
    LmExpr, LmModel, LmSol, LmStatus, LmVref,
    error::{CapiError, guard},
};

/// Evaluate the model for a given solution.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `solution` must be a valid model handle created by this library.
/// `out` must point to writable `LmSol` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_evaluate_solution(
    model: *const LmModel,
    solution: *const LmSol,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let sol = unsafe { (solution as *const Solution).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("solution"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let res = model
            .evaluate_solution(sol)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(res)) as *mut LmSol;
        Ok(())
    })
}

/// Evaluate the model for a given solution with a tolerance.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `solution` must be a valid model handle created by this library.
/// `tol` must be a valid `usize`, `tol == NULL` means use the default tolerance.
/// `out` must point to writable `LmSol` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_evaluate_solution_with_tol(
    model: *const LmModel,
    solution: *const LmSol,
    tol: *const f64,
    out: *mut *mut LmSol,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let sol = unsafe { (solution as *const Solution).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("solution"))?;
        let tol = unsafe { tol.as_ref() }.copied();
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let res = model
            .evaluate_solution_with_tol(sol, tol)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(res)) as *mut LmSol;
        Ok(())
    })
}

/// Substitute the variable with a given expression.
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
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let target = unsafe { (target as *const VarRef).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("target"))?;
        let replacement = unsafe { (replacement as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("replacement"))?;

        model
            .substitute(target, replacement)
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;

        Ok(())
    })
}
