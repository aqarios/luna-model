//! Operator implementations involving constraint collections.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;
use std::collections::HashMap;

use crate::{constraint::ConstraintCollection, traits::TryIndex};

impl ConstraintCollection {
    /// Evaluates every constraint against a name-addressable sample.
    ///
    /// The returned map preserves constraint names but not insertion order
    /// because it uses [`HashMap`].
    pub fn evaluate_sample<S>(
        &self,
        sample: &S,
        tol: Option<f64>,
    ) -> LunaModelResult<HashMap<String, bool>>
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

    /// Evaluates every constraint against a dense variable-value lookup vector.
    pub fn evaluate_sample_quick(
        &self,
        lu: &[Bias],
        tol: Option<f64>,
    ) -> LunaModelResult<HashMap<String, bool>> {
        self.iter()
            .map(
                |(name, constr)| match constr.evaluate_sample_quick(lu, tol) {
                    Ok(val) => Ok((name.clone(), val)),
                    Err(e) => Err(e),
                },
            )
            .collect::<LunaModelResult<HashMap<_, _>>>()
    }
}
