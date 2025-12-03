use super::PyVariable;
use crate::expression::PyExpression;
use pyo3::prelude::FromPyObject;

#[derive(FromPyObject)]
pub enum OpsOther {
    Expr(PyExpression),
    Var(PyVariable),
    Float(f64),
    Int(usize),
}
