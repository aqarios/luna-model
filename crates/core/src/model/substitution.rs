//! Variable-substitution helpers for models.

use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::{
    expression::Expression,
    ops::{EnvIdexable, foreign_env_error},
    variable::VarRef,
};

use super::Model;

impl Model {
    /// Substitutes a variable throughout the objective and all constraints.
    ///
    /// If the replacement no longer references the original variable, the now
    /// unused variable is removed from the environment.
    ///
    /// Fails without changing the model if `target` or `replacement` belong to
    /// a different environment than the model.
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        if target.env_id() != self.env_id() {
            let name = target.name().unwrap_or_default();
            return Err(LunaModelError::DifferentEnvironmentsIn(
                format!("substitution target '{name}' belongs to a different environment").into(),
            ));
        }
        if replacement.env_id() != self.env_id() {
            return Err(foreign_env_error("substitution replacement", replacement));
        }
        self.objective = self.objective.substitute(target, replacement)?;
        self.constraints.substitute(target, replacement)?;
        if !replacement.contains(target) {
            self.environment.remove(target);
        }
        Ok(())
    }
}
