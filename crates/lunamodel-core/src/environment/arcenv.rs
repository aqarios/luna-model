use parking_lot::RwLock;
use std::sync::Arc;

use super::Environment;
use crate::variable::{LazyBounds, VarRef};

use lunamodel_error::LunaModelResult;
use lunamodel_types::{EnvIdx, Vtype};

#[derive(Debug, Clone)]
pub struct ArcEnv {
    env: Arc<RwLock<Environment>>,
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
    pub fn default() -> Self {
        Self {
            env: Arc::new(RwLock::new(Environment::default())),
        }
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
}
