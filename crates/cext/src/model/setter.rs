use crate::utils::{mut_from_c, ref_from_c, str_from_c};
use std::ffi::c_char;

use luna_model::core::{ConstraintCollection, Expression, Model};

use crate::error::guard;
use crate::utils::sense_from_c;
use crate::{LmCC, LmExpr, LmModel, LmSense, LmStatus};

/// Sets the model name.
///
/// `name` must be non-null and valid UTF-8.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_set_name(model: *mut LmModel, name: *const c_char) -> LmStatus {
    guard(|| {
        let model = unsafe { mut_from_c(model as *mut Model, "model") }?;

        let name = unsafe { str_from_c(name, "name") }?;

        model.name = name.to_owned();

        Ok(())
    })
}

/// Sets the model sense.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_set_sense(model: *mut LmModel, sense: LmSense) -> LmStatus {
    guard(|| {
        let model = unsafe { mut_from_c(model as *mut Model, "model") }?;

        model.sense = sense_from_c(sense);

        Ok(())
    })
}

/// Sets the objective of the model.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `expr` must point to a valid `LmExpr` handle
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_set_objective(
    model: *mut LmModel,
    expr: *const LmExpr,
) -> LmStatus {
    guard(|| {
        let model = unsafe { mut_from_c(model as *mut Model, "model") }?;
        let expr = unsafe { ref_from_c(expr as *const Expression, "expr") }?;
        model.objective = expr.clone();
        Ok(())
    })
}

/// Sets the constraints of the model.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `cc` must point to a valid `LmCC` handle
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_set_constraints(
    model: *mut LmModel,
    cc: *const LmCC,
) -> LmStatus {
    guard(|| {
        let model = unsafe { mut_from_c(model as *mut Model, "model") }?;
        let cc = unsafe { ref_from_c(cc as *const ConstraintCollection, "cc") }?;
        model.constraints = cc.clone();
        Ok(())
    })
}
