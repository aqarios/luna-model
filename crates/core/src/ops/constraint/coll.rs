use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;
use std::collections::HashMap;

use crate::{constraint::ConstraintCollection, traits::TryIndex};

impl ConstraintCollection {
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<HashMap<String, bool>>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        self
            .iter()
            .map(|(name, constr)| match constr.evaluate_sample(sample) {
                Ok(val) => Ok((name.clone(), val)),
                Err(e) => Err(e),
            })
            .collect::<LunaModelResult<HashMap<_, _>>>()
    }

    pub fn evaluate_sample_quick(&self, lu: &[Bias]) -> LunaModelResult<HashMap<String, bool>> {
        self
            .iter()
            .map(|(name, constr)| match constr.evaluate_sample_quick(lu) {
                Ok(val) => Ok((name.clone(), val)),
                Err(e) => Err(e),
            })
            .collect::<LunaModelResult<HashMap<_, _>>>()
    }
}
