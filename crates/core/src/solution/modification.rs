use indexmap::IndexMap;
use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::solution::{Column, col::Assignment};

use super::Solution;

impl Solution {
    pub fn push(
        &mut self,
        sample: IndexMap<String, Assignment>,
        counts: usize,
        energy: Option<Bias>,
    ) -> LunaModelResult<()> {
        // First, let's check that the keys of the sample and the samples are equal.
        self.counts.push(counts);
        energy.and_then(|e| self.raw_energies.as_mut().and_then(|r| Some(r.push(e))));
        for (key, a) in sample {
            if let Some(col) = self.samples.get_mut(&key) {
                col.push(a)?;
            } else {
                self.samples.insert(key, Column::with(a));
            }
        }

        Ok(())
    }

    pub fn push_vec(&mut self, sample: Vec<Bias>, counts: usize, energy: Option<Bias>) {
        _ = sample;
        _ = counts;
        _ = energy;
    }

    pub fn add_binary(&mut self, var: String) {
        self.samples.insert(var, Column::empty_binary());
    }

    pub fn add_spin(&mut self, var: String) {
        self.samples.insert(var, Column::empty_spin());
    }

    pub fn add_integer(&mut self, var: String) {
        self.samples.insert(var, Column::empty_integer());
    }

    pub fn add_real(&mut self, var: String) {
        self.samples.insert(var, Column::empty_real());
    }
}
