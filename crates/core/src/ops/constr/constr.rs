use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{constraint::Constraint, traits::TryIndex};

impl Constraint {
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<bool>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        let lhs = self.lhs.evaluate_sample(sample)?;
        Ok(self.comparator.evaluate(lhs, self.rhs))
    }
}
