use crate::core::{Bound, Bounds, LazyBounds};
use derive_more::{Deref, DerefMut};
use pyo3::{
    exceptions::{PyRuntimeError, PyTypeError},
    prelude::*,
    IntoPyObjectExt, PyTypeInfo,
};

/// Represents bounds for a variable (only supported for real and integer variables).
///
/// A `Bounds` object defines the valid interval for a variable. Bounds are inclusive,
/// and can be partially specified by providing only a lower or upper limit. If neither
/// is specified, the variable is considered unbounded.
///
/// Parameters
/// ----------
/// lower : float, optional
///     Lower bound of the variable. Defaults to negative infinity if not specified.
/// upper : float, optional
///     Upper bound of the variable. Defaults to positive infinity if not specified.
///
/// Examples
/// --------
/// >>> from luna_quantum import Bounds
/// >>> Bounds(-1.0, 1.0)
/// Bounds { lower: -1, upper: 1 }
///
/// >>> Bounds(lower=0.0)
/// Bounds { lower: -1, upper: unlimited }
///
/// >>> Bounds(upper=10.0)
/// Bounds { lower: unlimited, upper: 1 }
///
/// Notes
/// -----
/// - Bounds are only meaningful for variables of type `Vtype.Real` or `Vtype.Integer`.
/// - If both bounds are omitted, the variable is unbounded.
#[cfg_attr(feature = "lq",      pyclass(name = "Bounds", module = "luna_quantum"))]
#[cfg_attr(not(feature = "lq"), pyclass(name = "Bounds", module = "aqmodels"))]
#[derive(Clone, Copy, Deref, DerefMut)]
pub struct PyBounds(pub LazyBounds);

impl Into<LazyBounds> for PyBounds {
    fn into(self) -> LazyBounds {
        self.0
    }
}

#[cfg_attr(feature = "lq",      pyclass(name = "Unbounded", module = "luna_quantum"))]
#[cfg_attr(not(feature = "lq"), pyclass(name = "Unbounded", module = "aqmodels"))]
#[derive(Debug, Clone, Copy)]
pub struct PyUnbounded;

#[pymethods]
impl PyUnbounded {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyRuntimeError::new_err(
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

impl<'s> FromPyObject<'s> for BoundValue {
    fn extract_bound(ob: &pyo3::Bound<'s, PyAny>) -> PyResult<Self> {
        if ob.is(&PyUnbounded::type_object(ob.py())) {
            Ok(BoundValue::Unbounded)
        } else if let Ok(maybe) = ob.extract::<Option<f64>>() {
            match maybe {
                Some(val) => Ok(BoundValue::Value(val)),
                None => Ok(BoundValue::None),
            }
        } else if let Ok(_) = ob.extract::<PyUnbounded>() {
            Ok(BoundValue::Unbounded)
        } else {
            Err(PyTypeError::new_err("Expected float, None, or 'Unbounded'"))
        }
    }
}

impl Into<Option<Bound>> for BoundValue {
    fn into(self) -> Option<Bound> {
        match self {
            Self::Unbounded => Some(Bound::Unbounded()),
            Self::Value(val) => Some(Bound::Some(val)),
            Self::None => None,
        }
    }
}

fn bound_into_py(py: Python, bound: Option<Bound>) -> PyResult<Py<PyAny>> {
    match bound {
        None => Ok(py.None()),
        Some(b) => match b {
            Bound::Some(val) => val.into_py_any(py),
            Bound::Unbounded() => "Unbounded".into_py_any(py),
        },
    }
}

#[pymethods]
impl PyBounds {
    /// Create bounds for a variable.
    ///
    /// See class-level docstring for full documentation.
    #[new]
    #[pyo3(signature=(lower=BoundValue::None, upper=BoundValue::None))]
    fn py_new(lower: BoundValue, upper: BoundValue) -> PyResult<Self> {
        Ok(PyBounds(Bounds::lazy(lower.into(), upper.into())))
    }

    #[getter]
    fn get_lower(&self, py: Python) -> PyResult<Py<PyAny>> {
        bound_into_py(py, self.lower)
    }

    #[getter]
    fn get_upper(&self, py: Python) -> PyResult<Py<PyAny>> {
        bound_into_py(py, self.upper)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
