use lunamodel_core::{
    Expression,
    prelude::{Constraint, VarRef},
};
use lunamodel_error::py::PyLunaModelError;
use lunamodel_types::Comparator;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use crate::{types::PyComparator, utils::OpsOther as OO};

use super::PyConstraint;

#[unwindable]
#[pymethods]
impl PyConstraint {
    #[new]
    #[pyo3(signature=(lhs, rhs, cmp, name=None))]
    pub fn py_new(lhs: OO, rhs: OO, cmp: PyComparator, name: Option<String>) -> PyResult<Self> {
        let cmp: Comparator = cmp.into();
        let constr = match (lhs, rhs) {
            (OO::Expr(l), OO::Expr(r)) => {
                let l_expr: Expression = l.into();
                let r_expr: Expression = r.into();
                let lhs = (l_expr - r_expr)?;
                Constraint::new(lhs, 0.0, cmp, name)
            }
            (OO::Expr(expr), OO::Var(var)) => {
                let l_expr: Expression = expr.into();
                let r_var: VarRef = var.v;
                let lhs = (l_expr - r_var)?;
                Constraint::new(lhs, 0.0, cmp, name)
            }
            (OO::Var(var), OO::Expr(expr)) => {
                let r_expr: Expression = expr.into();
                let l_var: VarRef = var.v;
                let lhs = (l_var - r_expr)?;
                Constraint::new(lhs, 0.0, cmp, name)
            }
            (OO::Expr(expr), OO::Num(bias)) => {
                let l_expr: Expression = expr.into();
                Constraint::new(l_expr, bias, cmp, name)
            }
            (OO::Num(bias), OO::Expr(expr)) => {
                let r_expr: Expression = expr.into();
                Constraint::new(-r_expr, -bias, cmp, name)
            }
            (OO::Var(l_var), OO::Var(r_var)) => {
                let l_expr: Expression = (l_var.v - r_var.v)?;
                Constraint::new(l_expr, 0.0, cmp, name)
            }
            (OO::Var(var), OO::Num(bias)) => {
                let l_expr: Expression = (Expression::empty(var.v.env.clone()) + var.v)?;
                Constraint::new(l_expr, bias, cmp, name)
            }
            (OO::Num(bias), OO::Var(var)) => {
                let r_expr: Expression = (Expression::empty(var.v.env.clone()) + var.v)?;
                Constraint::new(-r_expr, -bias, cmp, name)
            }
            (OO::Num(_), OO::Num(_)) => {
                return Err(PyLunaModelError::new_err(
                    "cannot create a constraint from two numerical values.",
                ));
            }
        }?;
        Ok(PyConstraint::new(constr))
    }
}
