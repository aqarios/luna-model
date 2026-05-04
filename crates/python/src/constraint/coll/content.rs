//! Shared backing storage for Python constraint collections.

use std::sync::Arc;

use lunamodel_core::{ConstraintCollection, Model};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

#[derive(Clone, Debug)]
pub enum PyConstraintCollectionContent {
    Coll(Arc<RwLock<ConstraintCollection>>),
    Model(Arc<RwLock<Model>>),
}

impl PyConstraintCollectionContent {
    pub fn read(&self) -> MappedRwLockReadGuard<'_, ConstraintCollection> {
        match self {
            Self::Coll(coll) => RwLockReadGuard::map(coll.read(), |coll| coll),
            Self::Model(model) => RwLockReadGuard::map(model.read(), |model| &model.constraints),
        }
    }

    pub fn write(&mut self) -> MappedRwLockWriteGuard<'_, ConstraintCollection> {
        match self {
            Self::Coll(coll) => RwLockWriteGuard::map(coll.write(), |coll| coll),
            Self::Model(model) => {
                RwLockWriteGuard::map(model.write(), |model| &mut model.constraints)
            }
        }
    }

    pub fn read_with<R>(&self, f: impl FnOnce(&ConstraintCollection) -> R) -> R {
        match self {
            Self::Coll(coll) => f(&coll.read_arc()),
            Self::Model(model) => f(&model.read_arc().constraints),
        }
    }

    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut ConstraintCollection) -> R) -> R {
        match self {
            Self::Coll(coll) => f(&mut coll.write_arc()),
            Self::Model(model) => f(&mut model.write_arc().constraints),
        }
    }
}
