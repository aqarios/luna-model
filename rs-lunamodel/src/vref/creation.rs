use super::VarRef;

use crate::{Env, Result};

impl VarRef {
    /// Creates a reference to the variable with index `id` in `env`.
    pub fn new(id: u32, env: Env) -> Result<Self> {
        out!(vref_new(id, @env), |out| Ok(Self::from_raw(out)))
    }
}
