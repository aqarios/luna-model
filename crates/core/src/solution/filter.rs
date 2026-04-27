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
        self.filter_by_mask(mask?.as_slice())
    }

    pub fn filter_by_mask(&self, mask: &[bool]) -> LunaModelResult<Self> {
        if self.len() != mask.len() {
            return Err(LunaModelError::Computation(
                "The mask's length does not match the number of samples".into(),
            ));
        }

        Ok(Solution {
            samples: self
                .samples
                .iter()
                .map(|(var, col)| (var.clone(), col.filter_by_mask(mask)))
                .collect::<IndexMap<_, _>>(),
            counts: self.counts.filter_by_mask(mask),
            raw_energies: self.raw_energies.as_ref().map(|e| e.filter_by_mask(mask)),
            obj_values: self.obj_values.as_ref().map(|o| o.filter_by_mask(mask)),
            feasible: self.feasible.as_ref().map(|f| f.filter_by_mask(mask)),
            constraints: self
                .constraints
                .iter()
                .map(|(cname, vs)| (cname.clone(), vs.filter_by_mask(mask)))
                .collect(),
            variable_bounds: self
                .variable_bounds
                .iter()
                .map(|(key, sample)| (key.clone(), sample.filter_by_mask(mask)))
                .collect(),
            timing: self.timing,
            sense: self.sense,
        })
    }
}
