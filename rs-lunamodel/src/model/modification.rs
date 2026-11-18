use lunamodel_types::{Sense, Vtype};

use crate::{ConstraintCollection, Expression, LazyBounds, Model, Result, VarRef};

impl Model {
    pub fn set_name(&mut self, name: String) -> Result<()> {
        check!(model_set_name(@mut self, !name))
    }

    pub fn set_sense(&mut self, sense: Sense) -> Result<()> {
        check!(model_set_sense(@mut self, ~sense))
    }

    /// Replaces the objective and optionally updates the optimization sense.
    pub fn set_objective(&mut self, objective: Expression, sense: Option<Sense>) -> Result<()> {
        check!(model_set_objective(@mut self, @objective, ?~sense))
    }

    pub fn set_constraints(&mut self, constraints: ConstraintCollection) -> Result<()> {
        check!(model_set_constraints(@mut self, @constraints))
    }

    pub fn add_var(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef> {
        out!(model_add_var(@mut self, !name, ~vtype, ?@bounds))
    }

    pub fn add_var_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef> {
        out!(model_add_var_with_fallback(@mut self, !name, ~vtype, ?@bounds))
    }
}
