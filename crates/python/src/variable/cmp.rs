use lunamodel_types::Comparator;
use pyo3::{PyResult, pymethods};

use super::PyVariable;
use crate::{constraint::PyConstraint, utils::OpsOther as OO};

#[pymethods]
impl PyVariable {
    fn __eq__(&self, other: OO) -> PyResult<PyConstraint> {
        PyConstraint::py_new(OO::Var(self.clone()), other, Comparator::Eq)
    }

    fn __le__(&self, other: OO) -> PyResult<PyConstraint> {
        PyConstraint::py_new(OO::Var(self.clone()), other, Comparator::Le)
    }

    fn __ge__(&self, other: OO) -> PyResult<PyConstraint> {
        PyConstraint::py_new(OO::Var(self.clone()), other, Comparator::Ge)
    }
}
