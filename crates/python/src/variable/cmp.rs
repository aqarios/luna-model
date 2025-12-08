use lunamodel_types::Comparator as Cmp;
use pyo3::{PyResult, pymethods};

use super::PyVariable;
use crate::{constraint::PyConstraint, utils::{OpsOther as OO, OtherOrTuple}};

#[pymethods]
impl PyVariable {
    fn __eq__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Var(self.clone()), o, Cmp::Eq, n)
    }

    fn __le__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Var(self.clone()), o, Cmp::Le, n)
    }

    fn __ge__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Var(self.clone()), o, Cmp::Ge, n)
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.v == other.v
    }
}
