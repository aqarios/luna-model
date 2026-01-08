use pyo3::pymethods;

use crate::PyConstraint;

use super::PyConstraintCollection;

#[pymethods]
impl PyConstraintCollection {
    // todo: actually this should also return a view like object.
    pub fn constraint(&self, key: &str) -> PyConstraint {
        let constr = &(&self.c.read_arc())[key];
        PyConstraint::new(constr.clone())
    }
}
