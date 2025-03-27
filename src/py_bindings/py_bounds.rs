use crate::core::Bounds;
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

/// The Bounds of a Variable. This is dependent on the type of the variable.
/// E.g., Spin and Binary variables do not accept explicit bounds setting.
/// Trying to do so will result in an error. Only Integer and Real types variables
/// are allowed to receive bounds.
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
