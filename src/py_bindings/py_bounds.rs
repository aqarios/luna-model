use crate::core::Bounds;
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(name = "Bounds")]
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
