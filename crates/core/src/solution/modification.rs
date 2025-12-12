use indexmap::IndexMap;
use lunamodel_types::Bias;

use crate::solution::Column;

use super::Solution;

impl Solution {
    pub fn push(&mut self, sample: IndexMap<String, Bias>, counts: usize, energy: Option<Bias>) {
        _ = sample;
    }

    pub fn push_vec(&mut self, sample: Vec<Bias>, counts: usize, energy: Option<Bias>) {
        _ = sample;
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
