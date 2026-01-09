use lunamodel_error::LunaModelResult;
use lunamodel_types::{Sense, Vtype};

use crate::{Expression, bounds::LazyBounds, variable::VarRef};

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

    pub fn add_var_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        enc: Option<&[u64]>,
    ) -> LunaModelResult<VarRef> {
        self.environment
            .insert_with_fallback(name, vtype, bounds, enc)
    }

    pub fn set_objective(&mut self, expr: Expression, sense: Option<Sense>) {
        if let Some(s) = sense {
            self.sense = s;
        }
        self.objective = expr;
    }
}
