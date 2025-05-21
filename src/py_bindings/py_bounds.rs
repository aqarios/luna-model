use crate::core::Bounds;
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

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
#[pyclass(name = "Bounds", module = "aqmodels")]
#[derive(Clone, Copy, Deref, DerefMut)]
pub struct PyBounds(Bounds);

impl Into<Bounds> for PyBounds {
    fn into(self) -> Bounds {
        self.0
    }
}

#[pymethods]
impl PyBounds {
    /// Create bounds for a variable.
    ///
    /// See class-level docstring for full documentation.
    #[new]
    #[pyo3(signature=(lower=None, upper=None))]
    fn py_new(lower: Option<f64>, upper: Option<f64>) -> PyResult<Self> {
        Ok(PyBounds(Bounds::new(lower, upper)))
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
