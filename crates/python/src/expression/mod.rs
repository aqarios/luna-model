mod access;
mod creation;
mod iteration;
mod ops;
// mod ser;

use lunamodel_core::prelude::{Expression, Model};
use parking_lot::RwLock;
use pyo3::pyclass;
use std::sync::Arc;

#[derive(Debug)]
pub enum PyExprContent {
    Expr(Arc<RwLock<Expression>>),
    Model(Arc<RwLock<Model>>),
}

impl Clone for PyExprContent {
    fn clone(&self) -> Self {
        match self {
            Self::Expr(e) => Self::Expr(e.clone()),
            Self::Model(e) => Self::Model(e.clone()),
        }
    }
}

#[pyclass]
#[repr(C)]
pub struct PyExpression {
    pub expr: PyExprContent,
}

impl From<Expression> for PyExpression {
    fn from(expr: Expression) -> Self {
        Self {
            expr: PyExprContent::Expr(Arc::new(RwLock::new(expr))),
        }
    }
}

impl From<Arc<RwLock<Model>>> for PyExpression {
    fn from(model: Arc<RwLock<Model>>) -> Self {
        Self {
            expr: PyExprContent::Model(model),
        }
    }
}

impl PyExpression {
    fn new(expr: Expression) -> Self {
        Self {
            expr: PyExprContent::Expr(Arc::new(RwLock::new(expr))),
        }
    }
}
