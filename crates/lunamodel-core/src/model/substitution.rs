use lunamodel_error::LunaModelResult;

use crate::{expression::Expression, variable::VarRef};

use super::Model;

impl Model {
    // TODO: this might need some improvements...

    /// Substitute every occurrence of a variable in the model’s objective and constraint expressions with another expression.
    ///
    /// Given a `Model` instance `self`, this method replaces all occurrences of `target`
    /// with `replacement` for the objective and each constraint.
    /// If any substitution would cross differing environments (e.g. captures from two
    /// different scopes), it returns a `DifferentEnvsError`.
    ///
    /// # Parameters
    /// - `target`: the variable reference to replace
    /// - `replacement`: the expression to insert in place of `target`
    ///
    /// # Returns
    /// - `Ok(())`: Unit type after substitution.
    /// - `Err(DifferentEnvsErr)`: if the environments of `self`, `target`, and `replacement`
    ///    are not compatible
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        self.objective = (&self.objective).substitute(target, replacement)?;
        self.constraints.substitute(target, replacement)?;
        if !replacement.contains(target) {
            self.environment.remove(target);
        }
        Ok(())
    }
}
