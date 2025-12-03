use lunamodel_types::Comparator;
use pyo3::{PyResult, pymethods};

use crate::utils::OpsOther as OO;

use super::PyConstraint;

#[pymethods]
impl PyConstraint {
    #[new]
    pub fn py_new(lhs: OO, rhs: OO, cmp: Comparator) -> PyResult<Self> {
        todo!()
    }
}
