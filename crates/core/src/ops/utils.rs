use crate::{Expression, prelude::VarRef};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::EnvIdx;

pub(crate) trait EnvIdexable {
    fn env_id(&self) -> EnvIdx;
}

pub(crate) fn check_envs<A: EnvIdexable, B: EnvIdexable>(a: &A, b: &B) -> LunaModelResult<()> {
    if a.env_id() != b.env_id() {
        Err(LunaModelError::DifferentEnvironments)
    } else {
        Ok(())
    }
}

impl EnvIdexable for Expression {
    fn env_id(&self) -> EnvIdx {
        self.env.id()
    }
}

impl EnvIdexable for VarRef {
    fn env_id(&self) -> EnvIdx {
        self.env.id()
    }
}
