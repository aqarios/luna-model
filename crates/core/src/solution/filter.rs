use indexmap::IndexMap;
use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::traits::FilterByMask;

use super::Solution;
use super::result::ResultView;

impl Solution {
    pub fn filter<F>(&self, f: F) -> LunaModelResult<Solution>
    where
        F: Fn(&ResultView) -> LunaModelResult<bool>,
    {
        let mask: LunaModelResult<Vec<bool>> = self.results().map(|r| f(&r)).collect();
        Ok(self.filter_by_mask(mask?.as_slice())?)
    }

    pub fn filter_by_mask(&self, mask: &[bool]) -> LunaModelResult<Self> {
        if self.n_samples != mask.len() {
            return Err(LunaModelError::Computation(
                "The mask's length does not match the number of samples".into(),
            ));
        }

        let mut res = Self::default();
        res.samples = self
            .samples
            .iter()
            .map(|(var, col)| (var.clone(), col.filter_by_mask(mask)))
            .collect::<IndexMap<_, _>>()
            .into();
        res.counts = self.counts.filter_by_mask(mask);
        res.raw_energies = self.raw_energies.as_ref().map(|e| e.filter_by_mask(mask));
        res.obj_values = self.obj_values.as_ref().map(|o| o.filter_by_mask(mask));
        res.feasible = self.feasible.as_ref().map(|f| f.filter_by_mask(mask));
        res.constraints = self.constraints.filter_by_mask(mask);
        res.variable_bounds = self
            .variable_bounds
            .iter()
            .map(|(key, sample)| (key.clone(), sample.filter_by_mask(mask)))
            .collect();
        res.timing = self.timing;
        res.n_samples = self.counts.len();
        res.sense = self.sense;
        Ok(res)
    }
}
