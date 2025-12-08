use pyo3::pymethods;

use super::PyExpression;

#[pymethods]
impl PyExpression {
    fn __str__(&self) -> String {
        unimplemented!()
    }

    fn __repr__(&self) -> String {
        unimplemented!()
    }
}
