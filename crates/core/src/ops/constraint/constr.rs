use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{constraint::Constraint, traits::TryIndex};

impl Constraint {
    /// Evaluates the constraint against a name-addressable sample.
    ///
    /// The sample only needs to provide `TryIndex<&str>` access to variable
    /// values; it does not need to be a concrete LunaModel solution type.
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<bool>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        let lhs = self.lhs.evaluate_sample(sample)?;
        Ok(self.comparator.evaluate(lhs, self.rhs))
    }

    /// Evaluates the constraint against a dense lookup vector indexed by variable id.
    ///
    /// This is the faster path used when the caller has already aligned values
    /// to environment order.
    pub fn evaluate_sample_quick(&self, lu: &[Bias]) -> LunaModelResult<bool> {
        let lhs = self.lhs.evaluate_sample_quick(lu)?;
        Ok(self.comparator.evaluate(lhs, self.rhs))
    }
}
