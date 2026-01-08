use std::ops::Mul;

use lunamodel_core::Expression;
use lunamodel_error::LunaModelResult;
use pyo3::{FromPyObject, PyResult, pymethods};

use crate::{PyExpression, PyVariable};

use super::PyModel;

#[derive(FromPyObject)]
enum Replacement {
    Expr(PyExpression),
    Var(PyVariable),
}

impl Replacement {
    fn as_expr(self) -> LunaModelResult<Expression> {
        match self {
            Replacement::Expr(expr) => Ok(expr.into()),
            Replacement::Var(var) => (&var.v).mul(1.0),
        }
    }
}

#[pymethods]
impl PyModel {
    fn substitute(&mut self, target: &PyVariable, replacement: Replacement) -> PyResult<()> {
        Ok(self
            .m
            .write_arc()
            .substitute(&target.v, &replacement.as_expr()?)?)
    }
}
