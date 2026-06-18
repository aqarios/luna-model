//! Low-level capsule and pointer FFI helpers for Python expression wrappers.

use std::{ffi::CStr, sync::Arc};

use lunamodel_core::{Expression, Model};
use lunamodel_unwind::*;
use parking_lot::RwLock;
use pyo3::{
    Bound, PyResult, Python,
    exceptions::PyValueError,
    pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::{PyExprContent, PyExpression, ffi::capsule_ffi::CapsuleFFI};

const CAPUSULE_NAME_EXPR: &CStr = c"builtins.capsule.PyExprContent.Expr";
const CAPUSULE_NAME_MODEL: &CStr = c"builtins.capsule.PyExprContent.Model";

impl<'py> CapsuleFFI<'py> for PyExprContent {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        match &self {
            Self::Expr(arc_expr) => {
                let capsule = PyCapsule::new_with_value(py, arc_expr.clone(), CAPUSULE_NAME_EXPR)?;
                Ok(capsule)
            }
            Self::Model(arc_model) => {
                let capsule =
                    PyCapsule::new_with_value(py, arc_model.clone(), CAPUSULE_NAME_MODEL)?;
                Ok(capsule)
            }
        }
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        if let Ok(ptr) = capsule.pointer_checked(Some(CAPUSULE_NAME_EXPR)) {
            let arc_expr = unsafe { ptr.cast::<Arc<RwLock<Expression>>>().as_ref().clone() };
            Ok(Self::Expr(arc_expr))
        } else if let Ok(ptr) = capsule.pointer_checked(Some(CAPUSULE_NAME_MODEL)) {
            let arc_model = unsafe { ptr.cast::<Arc<RwLock<Model>>>().as_ref().clone() };
            Ok(Self::Model(arc_model))
        } else {
            Err(PyValueError::new_err(
                "input is an unexpected capsule type.",
            ))
        }
    }
}

#[unwindable]
#[pymethods]
impl PyExpression {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.expr.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(Self {
            expr: PyExprContent::from_capsule(capsule)?,
        })
    }
}
