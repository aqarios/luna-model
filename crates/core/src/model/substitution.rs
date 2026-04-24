//! Variable-substitution helpers for models.

use lunamodel_error::LunaModelResult;

use crate::{expression::Expression, variable::VarRef};

use super::Model;

impl Model {
    /// Substitutes a variable throughout the objective and all constraints.
    ///
    /// If the replacement no longer references the original variable, the now
    /// unused variable is removed from the environment.
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        self.objective = self.objective.substitute(target, replacement)?;
        self.constraints.substitute(target, replacement)?;
        if !replacement.contains(target) {
            self.environment.remove(target);
        }
        Ok(())
    }
}
