use super::PyVariable;
use crate::expression::PyExpression;
use pyo3::prelude::FromPyObject;

#[derive(FromPyObject)]
pub enum OpsOther {
    Expr(PyExpression),
    Var(PyVariable),
    Num(f64),
    // Int(usize),
}

#[derive(FromPyObject)]
pub enum OtherOrTuple {
    Other(OpsOther),
    Tuple((OpsOther, String)),
}

impl Into<(OpsOther, Option<String>)> for OtherOrTuple {
    fn into(self) -> (OpsOther, Option<String>) {
        match self {
            OtherOrTuple::Other(o) => (o, None),
            OtherOrTuple::Tuple((o, n)) => (o, Some(n)),
        }
    }
}
