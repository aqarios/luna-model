use lunamodel_error::py::PyLunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyVariable;
use crate::{
    constraint::PyConstraint,
    utils::{OpsOther as OO, OtherOrTuple},
    types::PyComparator as Cmp,
};

#[unwindable]
#[pymethods]
impl PyVariable {
    fn __eq__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        if let OO::Var(_) = o {
            return Err(PyLunaModelError::new_err(
                "cannot use '__eq__' on two PyVariables directly. Use '.is_equal'",
            ));
        };
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
