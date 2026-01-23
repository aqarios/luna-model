use lunamodel_types::{Bias, Comparator};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyConstraint;
use crate::expression::PyExpression;

#[unwindable]
#[pymethods]
impl PyConstraint {
    #[getter]
    fn name(&self) -> String {
        self.c.read_arc().name().to_string()
    }

    #[getter]
    fn lhs(&self) -> PyExpression {
        PyExpression::new(self.c.read_arc().lhs.clone())
    }

    #[getter]
    fn rhs(&self) -> Bias {
        self.c.read_arc().rhs
    }

    #[getter]
    fn comparator(&self) -> Comparator {
        self.c.read_arc().comparator
    }
}
