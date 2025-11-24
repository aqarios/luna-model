use crate::environment::ArcEnv;
use lunamodel_types::VarIdx;

#[derive(Debug, Clone)]
pub struct VarRef {
    pub(crate) id: VarIdx,
    env: ArcEnv,
}

impl VarRef {
    pub fn new(id: VarIdx, env: ArcEnv) -> VarRef {
        Self { id, env }
    }
}
