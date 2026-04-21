use lunamodel_error::py::PyLunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyVariable;
use crate::{
    args::PyVarArg,
    constraint::PyConstraint,
    types::PyComparator as Cmp,
    utils::{OpsOther as OO, OtherOrTuple},
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
        PyConstraint::py_new(OO::Var(PyVarArg(self.clone())), o, Cmp::Eq, n)
    }

    fn __le__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Var(PyVarArg(self.clone())), o, Cmp::Le, n)
    }

    fn __ge__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Var(PyVarArg(self.clone())), o, Cmp::Ge, n)
    }

    fn is_equal(&self, other: PyVarArg) -> bool {
        self.v == other.v
    }
}
