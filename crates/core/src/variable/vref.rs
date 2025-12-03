use crate::environment::ArcEnv;
use lunamodel_error::LunaModelResult;
use lunamodel_types::VarIdx;

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
}
