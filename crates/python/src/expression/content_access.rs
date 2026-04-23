use lunamodel_core::prelude::Expression;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard,
};

use super::PyExprContent;

impl PyExprContent {
    pub fn read(&self) -> MappedRwLockReadGuard<'_, Expression> {
        match self {
            PyExprContent::Expr(expr) => RwLockReadGuard::map(expr.read(), |expr| expr),
            PyExprContent::Model(model) => {
                RwLockReadGuard::map(model.read(), |model| &model.objective)
            }
        }
    }

    pub fn write(&mut self) -> MappedRwLockWriteGuard<'_, Expression> {
        match self {
            PyExprContent::Expr(expr) => RwLockWriteGuard::map(expr.write(), |expr| expr),
            PyExprContent::Model(model) => {
                RwLockWriteGuard::map(model.write(), |model| &mut model.objective)
            }
        }
    }

    pub fn read_with<R>(&self, f: impl FnOnce(&Expression) -> R) -> R {
        match self {
            PyExprContent::Expr(expr) => f(&expr.read_arc()),
            PyExprContent::Model(model) => f(&model.read_arc().objective),
        }
    }

    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut Expression) -> R) -> R {
        match self {
            PyExprContent::Expr(expr) => f(&mut expr.write_arc()),
            PyExprContent::Model(model) => f(&mut model.write_arc().objective),
        }
    }
}
