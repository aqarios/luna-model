use lunamodel_types::Vtype;

use crate::{Env, Result, VarRef};

impl Env {
    pub fn len(&self) -> Result<usize> {
        out!(env_len(@self))
    }

    pub fn id(&self) -> Result<u128> {
        out!(env_id(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(env_is_empty(@self))
    }

    pub fn vars(&self) -> Result<Vec<VarRef>> {
        out!(env_vars(@self))
    }

    pub fn lookup(&self, name: &str) -> Result<VarRef> {
        out!(env_lookup(@self, !name))
    }

    pub fn get(&self, id: u32) -> Result<VarRef> {
        out!(env_get(@self, id))
    }

    pub fn contains(&self, name: &str) -> Result<bool> {
        out!(env_contains(@self, !name))
    }

    pub fn vtype_of(&self, name: &str) -> Result<Vtype> {
        out!(env_vtype_of(@self, !name))
    }
}
