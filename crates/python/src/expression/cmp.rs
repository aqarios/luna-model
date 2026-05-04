//! Equality and semantic-comparison helpers for Python expressions.

use lunamodel_core::prelude::ContentEquality;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyExpression;
use crate::{
    PyConstraint,
    args::PyExprArg,
    types::PyComparator as Cmp,
    utils::{OpsOther as OO, OtherOrTuple},
};

#[unwindable]
#[pymethods]
impl PyExpression {
    fn __eq__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Expr(PyExprArg(self.clone())), o, Cmp::Eq, n)
    }

    fn __le__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Expr(PyExprArg(self.clone())), o, Cmp::Le, n)
    }

    fn __ge__(&self, other: OtherOrTuple) -> PyResult<PyConstraint> {
        let (o, n) = other.into();
        PyConstraint::py_new(OO::Expr(PyExprArg(self.clone())), o, Cmp::Ge, n)
    }

    fn is_equal(&self, other: PyExprArg) -> bool {
        self.expr == other.expr
    }

    fn equal_contents(&self, other: PyExprArg) -> bool {
        self.expr.equal_contents(&other.expr)
    }
}
