use std::fmt::Display;

use crate::{
    PyEnvironment,
    args::{PyEnvArg, PyExprArg, PyModelArg, PyVarArg},
};
use pyo3::{
    PyResult,
    exceptions::PyValueError,
    prelude::{FromPyObject, PyErr},
};

use pyo3::{Py, PyAny, Python};

#[derive(Debug, Clone)]
pub enum VarKey {
    Str(String),
    Var(PyVarArg),
}

#[derive(Debug, FromPyObject)]
pub enum OpsOther {
    Expr(PyExprArg),
    Var(PyVarArg),
    Num(f64),
    // Int(usize),
}

#[derive(FromPyObject)]
pub enum OtherOrTuple {
    Other(OpsOther),
    Tuple((OpsOther, String)),
}

impl From<OtherOrTuple> for (OpsOther, Option<String>) {
    fn from(val: OtherOrTuple) -> Self {
        match val {
            OtherOrTuple::Other(o) => (o, None),
            OtherOrTuple::Tuple((o, n)) => (o, Some(n)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PyUsize(pub usize);

impl From<PyUsize> for usize {
    fn from(val: PyUsize) -> Self {
        val.0
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

pub fn as_usize_from_pyany(py: Python, n: Py<PyAny>) -> PyResult<usize> {
    let n: isize = n.extract(py)?;
    if n < 0 {
        Err(PyValueError::new_err(format!(
            "Expected a non-negative number, received: {n}"
        )))
    } else {
        Ok(n as usize)
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

pub fn retrieve_environment(
    env: Option<PyEnvArg>,
    model: &Option<PyModelArg>,
) -> PyResult<PyEnvironment> {
    if let Some(model) = model {
        Ok(model.m.read_arc().environment.clone().into())
    } else {
        env.try_into()
    }
}
