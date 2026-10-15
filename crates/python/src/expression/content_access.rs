//! Guard and callback-based access helpers for [`PyExprContent`].
//!
//! The callback helpers are usually the more ergonomic choice in binding code
//! because they avoid leaking concrete lock-guard types into generic call sites.

use lunamodel_core::prelude::Expression;
use parking_lot::{MappedRwLockReadGuard, RwLockReadGuard};

use super::PyExprContent;

impl PyExprContent {
    /// Borrow the wrapped expression as a mapped read guard.
    ///
    /// When the content is model-backed this maps the model lock to its
    /// objective field.
    pub fn read(&self) -> MappedRwLockReadGuard<'_, Expression> {
        match self {
            Self::Expr(expr) => RwLockReadGuard::map(expr.read(), |expr| expr),
            Self::Model(model) => RwLockReadGuard::map(model.read(), |model| model.objective()),
        }
    }

    /// Run a closure with a shared borrow of the wrapped expression.
    ///
    /// This is the preferred helper when the caller only needs transient access
    /// and wants to stay generic over model-backed and expression-backed cases.
    pub fn read_with<R>(&self, f: impl FnOnce(&Expression) -> R) -> R {
        match self {
            Self::Expr(expr) => f(&expr.read_arc()),
            Self::Model(model) => f(model.read_arc().objective()),
        }
    }
}
