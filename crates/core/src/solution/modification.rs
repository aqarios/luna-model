use indexmap::IndexMap;
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, Vtype};

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
        todo!("implement push_vec in modification.rs")
    }

    pub fn add_empty_binary(&mut self, var: String) {
        self.samples.insert(var, Column::empty_binary());
    }

    pub fn add_empty_spin(&mut self, var: String) {
        self.samples.insert(var, Column::empty_spin());
    }

    pub fn add_empty_integer(&mut self, var: String) {
        self.samples.insert(var, Column::empty_integer());
    }

    pub fn add_empty_real(&mut self, var: String) {
        self.samples.insert(var, Column::empty_real());
    }

    pub fn add_binary(&mut self, var: String, data: Vec<f64>) {
        self.samples.insert(var, Column::binary(data));
    }

    pub fn add_spin(&mut self, var: String, data: Vec<f64>) {
        self.samples.insert(var, Column::spin(data));
    }

    pub fn add_integer(&mut self, var: String, data: Vec<f64>) {
        self.samples.insert(var, Column::integer(data));
    }

    pub fn add_real(&mut self, var: String, data: Vec<f64>) {
        self.samples.insert(var, Column::real(data));
    }

    pub fn add_col(&mut self, vtype: Vtype, var: String, data: Vec<f64>) {
        match vtype {
            Vtype::Binary => self.add_binary(var, data),
            Vtype::Spin => self.add_spin(var, data),
            Vtype::Integer => self.add_integer(var, data),
            Vtype::Real => self.add_real(var, data),
            Vtype::InvertedBinary => (),
        }
    }

    pub fn remove_col(&mut self, var: String) {
        self.samples.shift_remove(&var);
    }
}
