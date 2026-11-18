use crate::{Constraint, Expression, Result, VarRef};

impl Constraint {
    pub fn set_name(&mut self, name: String) -> Result<()> {
        check!(constr_set_name(@mut self, !name))
    }

    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> Result<()> {
        check!(constr_substitute(@mut self, @target, @replacement))
    }
}
