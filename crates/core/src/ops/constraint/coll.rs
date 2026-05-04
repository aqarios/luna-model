use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;
use std::collections::HashMap;

use crate::{constraint::ConstraintCollection, traits::TryIndex};

impl ConstraintCollection {
    pub fn evaluate_sample<S>(&self, sample: &S, tol: Option<f64>) -> LunaModelResult<HashMap<String, bool>>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        self.iter()
            .map(|(name, constr)| match constr.evaluate_sample(sample, tol) {
                Ok(val) => Ok((name.clone(), val)),
                Err(e) => Err(e),
            })
            .collect::<LunaModelResult<HashMap<_, _>>>()
    }

    pub fn evaluate_sample_quick(&self, lu: &[Bias], tol: Option<f64>) -> LunaModelResult<HashMap<String, bool>> {
        self.iter()
            .map(|(name, constr)| match constr.evaluate_sample_quick(lu, tol) {
                Ok(val) => Ok((name.clone(), val)),
                Err(e) => Err(e),
            })
            .collect::<LunaModelResult<HashMap<_, _>>>()
    }
}
