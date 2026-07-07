use lunamodel_types::Vtype;

use crate::{Env, LazyBounds, Result, VarRef};

impl Env {
    pub fn insert(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef> {
        out!(env_insert(@mut self, !name, ~vtype, ?@bounds))
    }

    pub fn insert_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef> {
        out!(env_insert_with_fallback(@mut self, !name, ~vtype, ?@bounds))
    }

    pub fn insert_inverted(&mut self, vref: &VarRef) -> Result<VarRef> {
        out!(env_insert_inverted(@mut self, @vref))
    }

    pub fn remove(&mut self, target: &VarRef) -> Result<()> {
        check!(env_remove(@mut self, @target))
    }
}
