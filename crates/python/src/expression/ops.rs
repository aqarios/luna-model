use pyo3::prelude::*;

use crate::{PyExpression, PyVariable, expression::ExprContent};

#[pymethods]
impl PyExpression {
    fn __mul__(&self, rhs: &PyVariable) -> PyResult<PyExpression> {
        match &self.expr {
            ExprContent::Expr(e) => Ok((e * &rhs.v).unwrap().into()),
            ExprContent::Model(m) => Ok((&m.read_arc().objective * &rhs.v).unwrap().into()),
        }
    }
}
