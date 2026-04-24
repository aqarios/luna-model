use lunamodel_error::LunaModelResult;
use lunamodel_types::{Sense, Vtype};

use crate::{Expression, bounds::LazyBounds, variable::VarRef};

use super::Model;

impl Model {
    /// Adds a variable to the model environment.
    pub fn add_var(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarRef> {
        self.environment.insert(name, vtype, bounds)
    }

    /// Adds a variable, deriving a fallback name if the requested name is unavailable.
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

    /// Replaces the objective expression and optionally updates the optimization sense.
    pub fn set_objective(&mut self, expr: Expression, sense: Option<Sense>) {
        if let Some(s) = sense {
            self.sense = s;
        }
        self.objective = expr;
    }
}
