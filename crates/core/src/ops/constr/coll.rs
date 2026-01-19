use hashbrown::HashMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{
    constraint::ConstraintCollection,
    traits::{TryIndex, Variables},
};

impl ConstraintCollection {
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<HashMap<String, bool>>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
        S: Variables,
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
