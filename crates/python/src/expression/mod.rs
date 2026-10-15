//! Python wrapper for expressions and expression-adjacent sparse views.
mod access;
mod cmp;
mod content;
mod content_access;
mod creation;
mod deepclone;
mod fns;
mod io;
mod iteration;
mod ops;
mod ser;

use lunamodel_core::prelude::{Expression, Model};
use parking_lot::{MappedRwLockReadGuard, RwLock};
use pyo3::pyclass;
use std::sync::Arc;

pub use content::PyExprContent;
pub use iteration::{PyConstant, PyExpressionIterator, PyHigherOrder, PyLinear, PyQuadratic};

/// Python-visible expression wrapper.
#[pyclass(from_py_object, subclass)]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PyExpression {
    /// Shared expression-or-model-objective backing storage.
    pub expr: PyExprContent,
}

impl From<Expression> for PyExpression {
    /// Wraps an owned core expression for Python.
    fn from(expr: Expression) -> Self {
        Self {
            expr: PyExprContent::Expr(Arc::new(RwLock::new(expr))),
        }
    }
}

impl From<PyExpression> for Expression {
    /// Clones the underlying core expression out of the Python wrapper.
    fn from(val: PyExpression) -> Self {
        val.expr.into()
    }
}

impl From<Arc<RwLock<Model>>> for PyExpression {
    /// Wraps a model objective as a Python expression view.
    fn from(model: Arc<RwLock<Model>>) -> Self {
        Self {
            expr: PyExprContent::Model(model),
        }
    }
}

impl From<&PyExpression> for PyExpression {
    /// Clones the shared expression backing handle.
    fn from(value: &PyExpression) -> Self {
        Self {
            expr: value.expr.clone(),
        }
    }
}

impl PyExpression {
    /// Creates a Python wrapper from an owned core expression.
    pub fn new(expr: Expression) -> Self {
        Self {
            expr: PyExprContent::Expr(Arc::new(RwLock::new(expr))),
        }
    }
}

impl PyExpression {
    /// Borrows the underlying expression for read-only access.
    pub fn read(&self) -> MappedRwLockReadGuard<'_, Expression> {
        self.expr.read()
    }

    /// Runs a closure against an immutable view of the expression.
    pub fn read_with<R>(&self, f: impl FnOnce(&Expression) -> R) -> R {
        self.expr.read_with(f)
    }
}
