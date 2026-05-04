//! Evaluation helpers for individual variable references.

use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::variable::VarRef;

impl VarRef {
    /// Evaluates whether `value` satisfies this variable's current bounds.
    ///
    /// This does not inspect any expression context; it is purely a metadata
    /// check against the variable's bound interval.
    pub fn evaluate(&self, value: Bias) -> LunaModelResult<bool> {
        self.check_living()?;
        let bounds = self.bounds()?;
        Ok(bounds.evaluate(value))
    }
}
