use pyo3::pymethods;

use crate::PyExpression;

use super::PyModel;

#[pymethods]
impl PyModel {

    #[getter]
    fn get_objective(&self) -> PyExpression {
        PyExpression::from(self.m.clone())
    }
}
