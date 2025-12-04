mod access;
mod cmp;
mod content;
mod creation;
mod iteration;
mod ops;
mod ser;

use lunamodel_core::prelude::{Expression, Model};
use parking_lot::RwLock;
use pyo3::pyclass;
use std::sync::Arc;

pub use content::PyExprContent;

#[pyclass]
#[repr(C)]
#[derive(Clone)]
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

impl Into<Expression> for PyExpression {
    fn into(self) -> Expression {
        self.expr.into()
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
    pub fn new(expr: Expression) -> Self {
        Self {
            expr: PyExprContent::Expr(Arc::new(RwLock::new(expr))),
        }
    }
}
