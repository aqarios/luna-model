use std::ops::Index;

use hashbrown::HashMap;
use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::constraint::ConstraintCollection;

impl ConstraintCollection {
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<HashMap<String, bool>>
    where
        for<'s> S: Index<&'s str, Output = Bias>,
    {
        Ok(self
            .iter()
            .map(|(name, constr)| match constr.evaluate_sample(sample) {
                Ok(val) => Ok((name.clone(), val)),
                Err(e) => Err(e),
            })
            .collect::<LunaModelResult<HashMap<_, _>>>()?)
    }
}
