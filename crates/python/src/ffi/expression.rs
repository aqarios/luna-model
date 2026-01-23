use std::{ffi::CStr, sync::Arc};

use lunamodel_core::{Expression, Model};
use lunamodel_unwind::unwindable;
use parking_lot::RwLock;
use pyo3::{
    Bound, PyResult, Python,
    exceptions::PyValueError,
    pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::unwind::unwind;
use crate::{PyExprContent, PyExpression};

const CAPUSULE_NAME_EXPR: &CStr = c"builtins.capsule.PyExprContent.Expr";
const CAPUSULE_NAME_MODEL: &CStr = c"builtins.capsule.PyExprContent.Model";

impl PyExprContent {
    pub fn to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        match &self {
            PyExprContent::Expr(arc_expr) => {
                let capsule =
                    PyCapsule::new(py, arc_expr.clone(), Some(CAPUSULE_NAME_EXPR.to_owned()))?;
                Ok(capsule)
            }
            PyExprContent::Model(arc_model) => {
                let capsule =
                    PyCapsule::new(py, arc_model.clone(), Some(CAPUSULE_NAME_MODEL.to_owned()))?;
                Ok(capsule)
            }
        }
    }

    pub fn from_capsule<'py>(capsule: &Bound<'py, PyCapsule>) -> PyResult<Self> {
        if let Ok(ptr) = capsule.pointer_checked(Some(CAPUSULE_NAME_EXPR)) {
            let arc_expr = unsafe { ptr.cast::<Arc<RwLock<Expression>>>().as_ref().clone() };
            Ok(PyExprContent::Expr(arc_expr))
        } else if let Ok(ptr) = capsule.pointer_checked(Some(CAPUSULE_NAME_MODEL)) {
            let arc_model = unsafe { ptr.cast::<Arc<RwLock<Model>>>().as_ref().clone() };
            Ok(PyExprContent::Model(arc_model))
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
    pub fn _from_capsule<'py>(capsule: &Bound<'py, PyCapsule>) -> PyResult<PyExpression> {
        Ok(PyExpression {
            expr: PyExprContent::from_capsule(capsule)?,
        })
    }
}
