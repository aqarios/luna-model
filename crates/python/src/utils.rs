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

/// Python-side key used when indexing variable collections.
#[derive(Debug, Clone)]
pub enum VarKey {
    Str(String),
    Var(PyVarArg),
}

/// Binary operator argument accepted by Python expression/variable APIs.
#[derive(Debug, FromPyObject)]
pub enum OpsOther {
    Expr(PyExprArg),
    Var(PyVarArg),
    Num(f64),
    // Int(usize),
}

/// Python helper type accepting either a plain operand or `(operand, name)`.
#[derive(FromPyObject)]
pub enum OtherOrTuple {
    Other(OpsOther),
    Tuple((OpsOther, String)),
}

impl From<OtherOrTuple> for (OpsOther, Option<String>) {
    /// Normalizes the Python helper enum into a single internal tuple shape.
    fn from(val: OtherOrTuple) -> Self {
        match val {
            OtherOrTuple::Other(o) => (o, None),
            OtherOrTuple::Tuple((o, n)) => (o, Some(n)),
        }
    }
}

/// Python-extracted unsigned index type that rejects negative integers.
#[derive(Clone, Copy, Debug)]
pub struct PyUsize(pub usize);

impl From<PyUsize> for usize {
    /// Unwraps the checked usize wrapper.
    fn from(val: PyUsize) -> Self {
        val.0
    }
}

impl From<usize> for PyUsize {
    /// Wraps a plain usize for Python-facing APIs.
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyUsize {
    type Error = PyErr;

    /// Extracts a non-negative Python integer.
    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let n: isize = obj.extract()?;
        Ok(PyUsize(as_usize(n)?))
    }
}

impl Display for PyUsize {
    /// Displays the wrapped usize.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Extracts a non-negative Python integer from a generic `PyAny` handle.
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

/// Validates that a signed Python integer can be used as `usize`.
pub fn as_usize(n: isize) -> PyResult<usize> {
    if n < 0 {
        Err(PyValueError::new_err(format!(
            "Expected a non-negative number, received: {n}"
        )))
    } else {
        Ok(n as usize)
    }
}

/// Resolves the environment to use for a Python API call.
///
/// If a model is supplied, its environment takes precedence. Otherwise the
/// explicit environment argument is required.
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
