use lunamodel_core::{Expression, ops::LmAddAssign};
use pyo3::prelude::*;

use crate::{PyExpression, PyVariable, expression::PyExprContent};

#[pymethods]
impl PyExpression {
    fn __mul__(&self, rhs: &PyVariable) -> PyResult<PyExpression> {
        match &self.expr {
            PyExprContent::Expr(e) => {
                let expr: &Expression = &e.read_arc();
                Ok((expr * &rhs.v).unwrap().into())
            }
            PyExprContent::Model(m) => Ok((&m.read_arc().objective * &rhs.v).unwrap().into()),
        }
    }

    fn __iadd__(&mut self, rhs: f64) -> PyResult<()> {
        match &self.expr {
            PyExprContent::Expr(e) => e.write_arc().add_assign(rhs).unwrap(),
            PyExprContent::Model(e) => e.write_arc().objective.add_assign(rhs).unwrap(),
        }
        Ok(())
    }
}
