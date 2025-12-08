use lunamodel_error::py::PyLunaModelError;
use lunamodel_types::Bound;
use pyo3::{
    BoundObject, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr, PyResult, PyTypeInfo,
    Python, pyclass, pymethods, types::PyAnyMethods,
};

#[pyclass(subclass)]
#[derive(Debug, Clone, Copy)]
pub struct PyUnbounded;

#[pymethods]
impl PyUnbounded {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyLunaModelError::new_err(
            "Unbounded cannot be instantiated directly. Use the `Unbounded` type.",
        ))
    }

    fn __repr__(&self) -> &'static str {
        "Unbounded"
    }

    fn __str__(&self) -> &'static str {
        "Unbounded"
    }
}

#[derive(Debug)]
pub enum BoundValue {
    Value(f64),
    None,
    Unbounded,
}

impl From<Bound> for BoundValue {
    fn from(value: Bound) -> Self {
        match value {
            Bound::Unbounded => Self::Unbounded,
            Bound::Bounded(v) => Self::Value(v),
        }
    }
}

impl From<Option<Bound>> for BoundValue {
    fn from(value: Option<Bound>) -> Self {
        match value {
            None => BoundValue::None,
            Some(val) => val.into(),
        }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for BoundValue {
    type Error = PyErr;
    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if obj.is(&PyUnbounded::type_object(obj.py())) {
            Ok(BoundValue::Unbounded)
        } else if let Ok(maybe) = obj.extract::<Option<f64>>() {
            match maybe {
                Some(val) => Ok(BoundValue::Value(val)),
                None => Ok(BoundValue::None),
            }
        } else if let Ok(_) = obj.extract::<PyUnbounded>() {
            Ok(BoundValue::Unbounded)
        } else {
            Err(PyLunaModelError::new_err(
                "Expected float, None, or 'Unbounded'",
            ))
        }
    }
}

impl<'py> IntoPyObject<'py> for BoundValue {
    type Target = PyAny;
    type Output = pyo3::Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Self::None => Ok(py.None().into_bound(py)),
            Self::Unbounded => Ok(PyUnbounded::type_object(py).into_py_any(py)?.into_bound(py)),
            Self::Value(val) => Ok(val.into_py_any(py)?.into_bound(py)),
        }
    }
}

impl Into<Option<Bound>> for BoundValue {
    fn into(self) -> Option<Bound> {
        match self {
            Self::Unbounded => Some(Bound::Unbounded),
            Self::Value(val) => Some(Bound::Bounded(val)),
            Self::None => None,
        }
    }
}
