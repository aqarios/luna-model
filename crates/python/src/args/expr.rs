use derive_more::Deref;
use lunamodel_core::Expression;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyExpression;

#[derive(Deref, Debug)]
pub struct PyExprArg(pub PyExpression);

impl Into<PyExpression> for PyExprArg {
    fn into(self) -> PyExpression {
        self.0
    }
}

impl Into<Expression> for PyExprArg {
    fn into(self) -> Expression {
        self.0.into()
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyExprArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyExpression>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_expr") {
            if let Ok(c) = inner.extract::<PyRef<'py, PyExpression>>() {
                return Ok(Self(c.clone()));
            }
        }

        Err(PyTypeError::new_err("Expected (Py)Expression"))
    }
}
