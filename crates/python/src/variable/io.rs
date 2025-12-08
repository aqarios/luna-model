use pyo3::pymethods;

use super::PyVariable;

#[pymethods]
impl PyVariable {
    fn __str__(&self) -> String {
        format!("{}", self.v)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.v)
    }
}
