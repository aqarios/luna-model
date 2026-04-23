use lunamodel_core::prelude::Expression;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard,
};

use super::PyExprContent;

impl PyExprContent {
    pub fn read(&self) -> MappedRwLockReadGuard<'_, Expression> {
        match self {
            Self::Expr(expr) => RwLockReadGuard::map(expr.read(), |expr| expr),
            Self::Model(model) => RwLockReadGuard::map(model.read(), |model| &model.objective),
        }
    }

    pub fn write(&mut self) -> MappedRwLockWriteGuard<'_, Expression> {
        match self {
            Self::Expr(expr) => RwLockWriteGuard::map(expr.write(), |expr| expr),
            Self::Model(model) => {
                RwLockWriteGuard::map(model.write(), |model| &mut model.objective)
            }
        }
    }

    pub fn read_with<R>(&self, f: impl FnOnce(&Expression) -> R) -> R {
        match self {
            Self::Expr(expr) => f(&expr.read_arc()),
            Self::Model(model) => f(&model.read_arc().objective),
        }
    }

    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut Expression) -> R) -> R {
        match self {
            Self::Expr(expr) => f(&mut expr.write_arc()),
            Self::Model(model) => f(&mut model.write_arc().objective),
        }
    }
}
