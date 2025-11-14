use crate::environment::Environment;
use lunamodel_types::VarIdx;

#[derive(Debug, Clone)]
pub struct VarRef<'a> {
    pub(crate) id: VarIdx,
    env: &'a Environment,
}

impl<'a> VarRef<'a> {
    pub fn new(id: VarIdx, env: &'a Environment) -> VarRef<'a> {
        Self { id, env }
    }
}
