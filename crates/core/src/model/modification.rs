use lunamodel_error::LunaModelResult;
use lunamodel_types::Vtype;

use crate::{bounds::LazyBounds, variable::VarRef};

use super::Model;

impl Model {
    pub fn add_var(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarRef> {
        self.environment.insert(name, vtype, bounds)
    }
}
