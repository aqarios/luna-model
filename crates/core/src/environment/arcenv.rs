use derive_more::Deref;
use parking_lot::RwLock;
use std::sync::Arc;

use super::Environment;
use crate::{bounds::LazyBounds, traits::ContentEquality, variable::VarRef};

use lunamodel_error::LunaModelResult;
use lunamodel_types::{EnvIdx, VarIdx, Vtype};

#[derive(Debug, Clone, Default, Deref)]
pub struct ArcEnv {
    pub env: Arc<RwLock<Environment>>,
}

impl From<Arc<RwLock<Environment>>> for ArcEnv {
    fn from(env: Arc<RwLock<Environment>>) -> Self {
        Self { env }
    }
}

impl From<Environment> for ArcEnv {
    fn from(env: Environment) -> Self {
        Self {
            env: Arc::new(RwLock::new(env)),
        }
    }
}

impl ArcEnv {
    pub fn deep_clone(&self) -> Self {
        self.env.read_arc().deep_clone().into()
    }

    pub fn len(&self) -> usize {
        self.env.read_arc().len()
    }

    pub fn is_empty(&self) -> bool {
        self.env.read_arc().is_empty()
    }

    pub fn vars(&self) -> Vec<VarRef> {
        // maybe we can return iter when passing closure for vref gen.
        self.env
            .read_arc()
            .vars()
            .map(|v| VarRef::new(v, self.clone()))
            .collect()
    }

    pub fn insert(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(
            self.env.write_arc().insert(name, vtype, bounds)?,
            self.clone(),
        ))
    }

    pub fn insert_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        enc: Option<&[u64]>,
    ) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(
            self.env
                .write_arc()
                .insert_with_fallback(name, vtype, bounds, enc)?,
            self.clone(),
        ))
    }

    pub fn insert_inverted(&mut self, base: &VarRef) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(
            self.env.write_arc().insert_inverted(base)?,
            self.clone(),
        ))
    }

    pub fn remove(&mut self, target: &VarRef) {
        self.env.write_arc().remove(target);
    }

    pub fn id(&self) -> EnvIdx {
        self.env.read_arc().id
    }

    pub fn lookup(&self, name: &str) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(self.env.read_arc().lookup(name)?, self.clone()))
    }

    pub fn get(&self, idx: VarIdx) -> VarRef {
        VarRef::new(idx, self.clone())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.env.read_arc().lookup.contains_key(name)
    }

    pub fn vtype_of(&self, name: &str) -> LunaModelResult<Vtype> {
        self.env.read_arc().vtype_of(name)
    }

    pub fn sort(&self, mut vars: Vec<String>) -> Vec<String> {
        let env = &self.env.read_arc();
        vars.sort_by_key(|a| env.lookup(a).unwrap());
        vars
    }
}

impl ContentEquality for ArcEnv {
    fn equal_contents(&self, other: &Self) -> bool {
        for v in self.vars() {
            let vname = v.name().unwrap();
            match other.lookup(&vname) {
                Err(_) => return false,
                Ok(o) => {
                    if v.vtype().unwrap() != o.vtype().unwrap() {
                        return false;
                    }
                    if v.bounds().unwrap() != o.bounds().unwrap() {
                        return false;
                    }
                }
            }
        }
        true
    }
}
