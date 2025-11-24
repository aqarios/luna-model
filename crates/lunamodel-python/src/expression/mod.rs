mod access;
mod creation;
mod iteration;
mod ser;

use lunamodel_core::prelude::{Expression, Model};
use parking_lot::RwLock;
use pyo3::pyclass;
use std::sync::Arc;

pub(crate) enum ExprContent {
    Expr(Expression),
    Model(Arc<RwLock<Model>>),
}

#[pyclass]
#[repr(transparent)]
pub struct PyExpression {
    pub expr: ExprContent,
}

impl From<Expression> for PyExpression {
    fn from(expr: Expression) -> Self {
        Self {
            expr: ExprContent::Expr(expr),
        }
    }
}

impl From<Arc<RwLock<Model>>> for PyExpression {
    fn from(model: Arc<RwLock<Model>>) -> Self {
        Self {
            expr: ExprContent::Model(model),
        }
    }
}
