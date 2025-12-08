use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{bounds::Bounds, environment::ArcEnv, traits::ContentEquality};
use lunamodel_error::LunaModelResult;
use lunamodel_types::{VarIdx, Vtype};

#[derive(Debug, Clone)]
pub struct VarRef {
    pub(crate) id: VarIdx,
    pub env: ArcEnv,
}

impl VarRef {
    pub fn new(id: VarIdx, env: ArcEnv) -> VarRef {
        Self { id, env }
    }

    pub fn id(&self) -> VarIdx {
        self.id
    }

    pub fn check_living(&self) -> LunaModelResult<()> {
        _ = self.env.read_arc().get(self.id)?;
        Ok(())
    }

    pub fn name(&self) -> LunaModelResult<String> {
        Ok(self.env.read_arc().get(self.id)?.name.0.clone())
    }

    pub fn vtype(&self) -> LunaModelResult<Vtype> {
        Ok(self.env.read_arc().get(self.id)?.vtype)
    }

    pub fn bounds(&self) -> LunaModelResult<Bounds> {
        Ok(self.env.read_arc().get(self.id)?.bounds)
    }

    pub fn hash(&self) -> LunaModelResult<u64> {
        let mut state = DefaultHasher::new();
        self.name()?.hash(&mut state);
        Ok(state.finish())
    }
}

impl PartialEq for VarRef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.env.is_equal_contents(&other.env)
    }
}
