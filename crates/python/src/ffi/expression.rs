//! C-handle capsule transport for Python expression wrappers.

use lunamodel_capi_handle::{
    ExpressionHandle, drop_handle, handle_from_storage, handle_into_storage,
};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_capi::LmExpr;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::{PyExprContent, PyExpression};

#[unwindable]
#[pymethods]
impl PyExpression {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let storage = match &self.expr {
            PyExprContent::Expr(expression) => ExpressionHandle::Expression(expression.clone()),
            PyExprContent::Model(model) => ExpressionHandle::Model(model.clone()),
        };
        let handle = handle_from_storage::<LmExpr>(storage);
        new_capsule(py, handle, drop_handle::<LmExpr>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmExpr>(&capsule)?;
        let storage = unsafe { handle_into_storage::<LmExpr>(handle) }
            .expect("a consumed non-null expression handle must contain storage");
        let expr = match storage {
            ExpressionHandle::Expression(expression) => PyExprContent::Expr(expression),
            ExpressionHandle::Model(model) => PyExprContent::Model(model),
        };
        Ok(Self { expr })
    }
}
