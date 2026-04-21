use std::sync::Arc;

use lunamodel_core::{
    ConstraintCollection, Model,
    prelude::{Constraint, ContentEquality},
};
use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::Encodable;
use lunamodel_types::Comparator;
use parking_lot::RwLock;
use pyo3::{PyResult, Python};

use crate::{
    PyConstraint,
    constraint::utils::{ConstraintsIn, NameIn, add_many_constraint},
};

#[derive(Clone, Debug)]
pub enum PyConstraintCollectionContent {
    Coll(Arc<RwLock<ConstraintCollection>>),
    Model(Arc<RwLock<Model>>),
}

impl PyConstraintCollectionContent {
    pub fn collection(self) -> ConstraintCollection {
        match self {
            Self::Model(m) => m.read_arc().constraints.clone(),
            Self::Coll(c) => c.read_arc().clone(),
        }
    }

    pub fn get(&self, key: &str) -> PyResult<PyConstraint> {
        Ok(match self {
            Self::Coll(coll) => coll.read_arc().get(key)?.clone().into(),
            Self::Model(m) => m.read_arc().constraints.get(key)?.clone().into(),
        })
    }

    pub fn ctypes(&self) -> Vec<Comparator> {
        match self {
            Self::Coll(coll) => coll.read_arc().ctypes().collect(),
            Self::Model(m) => m.read_arc().constraints.ctypes().collect(),
        }
    }

    pub fn equal_contents(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Coll(l), Self::Coll(r)) => l.read_arc().equal_contents(&r.read_arc()),
            (Self::Model(l), Self::Coll(r)) => {
                l.read_arc().constraints.equal_contents(&r.read_arc())
            }
            (Self::Coll(l), Self::Model(r)) => {
                l.read_arc().equal_contents(&r.read_arc().constraints)
            }
            (Self::Model(l), Self::Model(r)) => l
                .read_arc()
                .constraints
                .equal_contents(&r.read_arc().constraints),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Coll(c) => c.read_arc().len(),
            Self::Model(m) => m.read_arc().constraints.len(),
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Coll(l), Self::Coll(r)) => l.read_arc().eq(&r.read_arc()),
            (Self::Model(l), Self::Coll(r)) => l.read_arc().constraints.eq(&r.read_arc()),
            (Self::Coll(l), Self::Model(r)) => l.read_arc().eq(&r.read_arc().constraints),
            (Self::Model(l), Self::Model(r)) => {
                l.read_arc().constraints.eq(&r.read_arc().constraints)
            }
        }
    }

    pub fn pyitems(&self) -> Vec<(String, PyConstraint)> {
        match self {
            Self::Coll(c) => c
                .read_arc()
                .iter()
                .map(|(name, constr)| (name.clone(), constr.into()))
                .collect(),
            Self::Model(m) => m
                .read_arc()
                .constraints
                .iter()
                .map(|(name, constr)| (name.clone(), constr.into()))
                .collect(),
        }
    }

    pub fn add_constraint(
        &mut self,
        constr: Constraint,
        name: Option<String>,
    ) -> LunaModelResult<String> {
        match self {
            Self::Coll(c) => c.write_arc().add_constraint(constr, name),
            Self::Model(m) => m.write_arc().constraints.add_constraint(constr, name),
        }
    }

    pub fn add_collection(
        &mut self,
        coll: Self,
        prefix: Option<String>,
    ) -> LunaModelResult<Vec<String>> {
        let collection = match coll {
            Self::Coll(c) => c.read_arc().clone(),
            Self::Model(m) => m.write_arc().constraints.clone(),
        };
        match self {
            Self::Coll(c) => c.write_arc().add_collection(collection, prefix),
            Self::Model(m) => m.write_arc().constraints.add_collection(collection, prefix),
        }
    }

    pub fn add_many_py(
        &mut self,
        py: Python,
        ccin: ConstraintsIn,
        nin: Option<NameIn>,
    ) -> LunaModelResult<Vec<String>> {
        match self {
            Self::Model(m) => add_many_constraint(py, &mut m.write_arc().constraints, ccin, nin),
            Self::Coll(c) => add_many_constraint(py, &mut c.write_arc(), ccin, nin),
        }
    }

    pub fn add_many(
        &mut self,
        others: impl Iterator<Item = (Constraint, Option<String>)>,
    ) -> LunaModelResult<Vec<String>> {
        match self {
            Self::Coll(c) => c.write_arc().add_many_constraints(others),
            Self::Model(m) => m.write_arc().constraints.add_many_constraints(others),
        }
    }

    pub fn set_constraint(&mut self, key: &str, constr: Constraint) -> LunaModelResult<()> {
        match self {
            Self::Coll(c) => c.write_arc().set_constraint(key, constr),
            Self::Model(m) => m.write_arc().constraints.set_constraint(key, constr),
        }
    }

    pub fn remove_constraint(&mut self, key: &str) -> LunaModelResult<()> {
        match self {
            Self::Coll(c) => c.write_arc().remove_constraint(key),
            Self::Model(m) => m.write_arc().constraints.remove_constraint(key),
        }
    }

    pub fn encode(&self, compress: Option<bool>, level: Option<i32>) -> LunaModelResult<Vec<u8>> {
        match self {
            Self::Coll(c) => c.read_arc().encode(compress, level),
            Self::Model(m) => m.read_arc().constraints.encode(compress, level),
        }
    }
}

impl Into<ConstraintCollection> for &PyConstraintCollectionContent {
    fn into(self) -> ConstraintCollection {
        match self {
            PyConstraintCollectionContent::Coll(c) => c.read_arc().clone(),
            PyConstraintCollectionContent::Model(m) => m.read_arc().constraints.clone(),
        }
    }
}
