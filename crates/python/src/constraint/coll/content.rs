//! Shared backing storage for Python constraint collections.

use std::sync::Arc;

use lunamodel_core::{Constraint, ConstraintCollection, Model};
use lunamodel_error::LunaModelResult;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};

#[derive(Clone, Debug)]
pub enum PyConstraintCollectionContent {
    Coll(Arc<RwLock<ConstraintCollection>>),
    Model(Arc<RwLock<Model>>),
}

/// Read access works on both variants. Writes go through the operations below,
/// so a model-backed collection is always modified via the checked `Model` API.
impl PyConstraintCollectionContent {
    pub fn read(&self) -> MappedRwLockReadGuard<'_, ConstraintCollection> {
        match self {
            Self::Coll(coll) => RwLockReadGuard::map(coll.read(), |coll| coll),
            Self::Model(model) => RwLockReadGuard::map(model.read(), |model| model.constraints()),
        }
    }

    pub fn read_with<R>(&self, f: impl FnOnce(&ConstraintCollection) -> R) -> R {
        match self {
            Self::Coll(coll) => f(&coll.read_arc()),
            Self::Model(model) => f(model.read_arc().constraints()),
        }
    }

    pub fn add_constraint(
        &mut self,
        constr: Constraint,
        name: Option<String>,
    ) -> LunaModelResult<String> {
        match self {
            Self::Coll(coll) => coll.write().add_constraint(constr, name),
            Self::Model(model) => model.write().add_constraint(constr, name),
        }
    }

    pub fn add_many_constraints(
        &mut self,
        constrs: Vec<(Constraint, Option<String>)>,
    ) -> LunaModelResult<Vec<String>> {
        match self {
            Self::Coll(coll) => coll.write().add_many_constraints(constrs.into_iter()),
            Self::Model(model) => model.write().add_many_constraints(constrs),
        }
    }

    pub fn set_constraint(&mut self, key: &str, constr: Constraint) -> LunaModelResult<()> {
        match self {
            Self::Coll(coll) => coll.write().set_constraint(key, constr),
            Self::Model(model) => model.write().set_constraint(key, constr),
        }
    }

    pub fn remove_constraint(&mut self, key: &str) -> LunaModelResult<()> {
        match self {
            Self::Coll(coll) => coll.write().remove_constraint(key),
            Self::Model(model) => model.write().remove_constraint(key),
        }
    }
}
