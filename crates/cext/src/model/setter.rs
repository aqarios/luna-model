use std::ffi::c_char;

use crate::{
    LmCC, LmExpr, LmModel, LmSense, LmStatus,
    error::guard,
    utils::sense_from_c,
    utils::{shared_from_c, storage_from_c, str_from_c},
};

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
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
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
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
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
        let expr = unsafe { storage_from_c(expr, "expr") }?.read().clone();
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
        model.objective = expr;
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
        let cc = unsafe { storage_from_c(cc, "cc") }?.read().clone();
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
        model.constraints = cc;
        Ok(())
    })
}
