use std::ffi::{CStr, c_char};

use luna_model::core::{ConstraintCollection, Expression, Model};

use crate::error::{CapiError, guard};
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
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;

        if name.is_null() {
            return Err(CapiError::null_pointer("name"));
        }

        let name = unsafe { CStr::from_ptr(name) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`name` must be valid UTF-8: {e}"),
            )
        })?;

        model.name = name.to_owned();

        Ok(())
    })
}

/// Sets the model sense.
///
/// `sense` must be non-null and valid UTF-8.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `sense` must point to a valid LmSense.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_set_sense(
    model: *mut LmModel,
    sense: *const LmSense,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;

        if sense.is_null() {
            return Err(CapiError::null_pointer("sense"));
        }

        model.sense = sense_from_c(unsafe { *sense });

        Ok(())
    })
}

/// Set the objective of the model.
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
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let expr = unsafe { (expr as *const Expression).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("expr"))?;
        model.objective = expr.clone();
        Ok(())
    })
}

/// Set the constraints of the model.
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
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let cc = unsafe { (cc as *const ConstraintCollection).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("cc"))?;
        model.constraints = cc.clone();
        Ok(())
    })
}
