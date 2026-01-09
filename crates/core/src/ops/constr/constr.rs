use std::ops::Index;

use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::constraint::Constraint;

impl Constraint {
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<bool>
    where
        for<'s> S: Index<&'s str, Output = Bias>,
    {
        let lhs = self.lhs.evaluate_sample(sample)?;
        Ok(self.comparator.evaluate(lhs, self.rhs))
    }
}
