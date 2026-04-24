//! Flexible Python argument extraction for expression-like inputs.

use derive_more::Deref;
use lunamodel_core::Expression;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyExpression;

#[derive(Deref, Debug)]
pub struct PyExprArg(pub PyExpression);

impl From<PyExprArg> for PyExpression {
    fn from(val: PyExprArg) -> Self {
        val.0
    }
}

impl From<PyExprArg> for Expression {
    fn from(val: PyExprArg) -> Self {
        val.0.into()
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyExprArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyExpression>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_expr")
            && let Ok(c) = inner.extract::<PyRef<'py, PyExpression>>()
        {
            return Ok(Self(c.clone()));
        }

        Err(PyTypeError::new_err("Expected (Py)Expression"))
    }
}
