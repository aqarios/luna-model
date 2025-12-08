use pyo3::pymethods;

use super::PyBounds;

#[pymethods]
impl PyBounds {
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
