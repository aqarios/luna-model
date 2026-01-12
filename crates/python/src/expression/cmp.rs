use lunamodel_core::prelude::ContentEquality;
use lunamodel_types::Comparator as Cmp;
use pyo3::{PyResult, pymethods};

use crate::{
    PyConstraint,
    utils::{OpsOther as OO, OtherOrTuple},
};

use super::PyExpression;

#[pymethods]
impl PyExpression {
    fn __eq__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Expr(self.clone()), o, Cmp::Eq, n)
    }

    fn __le__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Expr(self.clone()), o, Cmp::Le, n)
    }

    fn __ge__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Expr(self.clone()), o, Cmp::Ge, n)
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.expr == other.expr
    }

    fn equal_contents(&self, other: &Self) -> bool {
        (&self.expr).equal_contents(&other.expr)
    }
}
