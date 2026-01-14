use std::fmt::Display;

use super::PyVariable;
use crate::expression::PyExpression;
use pyo3::{
    PyResult, exceptions::PyValueError, prelude::{FromPyObject, PyErr}
};

#[derive(Debug, FromPyObject)]
pub enum OpsOther {
    Expr(PyExpression),
    Var(PyVariable),
    Num(f64),
    // Int(usize),
}

#[derive(FromPyObject)]
pub enum OtherOrTuple {
    Other(OpsOther),
    Tuple((OpsOther, String)),
}

impl Into<(OpsOther, Option<String>)> for OtherOrTuple {
    fn into(self) -> (OpsOther, Option<String>) {
        match self {
            OtherOrTuple::Other(o) => (o, None),
            OtherOrTuple::Tuple((o, n)) => (o, Some(n)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PyUsize(pub usize);

impl Into<usize> for PyUsize {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for PyUsize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyUsize {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let n: isize = obj.extract()?;
        Ok(PyUsize(as_usize(n)?))
   }
}

impl Display for PyUsize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


pub fn as_usize(n: isize) -> PyResult<usize> {
        if n < 0 {
            Err(PyValueError::new_err(format!(
                "Expected a non-negative number, received: {n}"
            )))
        } else {
            Ok(n as usize)
        }
}