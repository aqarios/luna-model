use lunamodel_core::prelude::Constraint;
use pyo3::pymethods;

use super::PyConstraint;

#[pymethods]
impl PyConstraint {
    fn __eq__(&self, other: &Self) -> bool {
        let lhs: &Constraint = &self.c.read_arc();
        let rhs: &Constraint = &other.c.read_arc();
        lhs == rhs
    }
}
