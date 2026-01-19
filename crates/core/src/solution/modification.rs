use hashbrown::HashMap;
use indexmap::IndexMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
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

    // pub fn push_vec(&mut self, sample: Vec<Bias>, counts: usize, energy: Option<Bias>) {
    //     _ = sample;
    //     _ = counts;
    //     _ = energy;
    //     todo!("implement push_vec in modification.rs")
    // }

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

    /// Combine duplicate samples to a single entry.
    pub fn combine_to_single(&mut self) -> LunaModelResult<()> {
        let mut rows: HashMap<String, usize> = HashMap::new();

        let mut to_rm: IndexMap<usize, Vec<usize>> = IndexMap::new();

        for rowidx in 0..self.n_samples() {
            let rowkey: String = self
                .variable_names()
                .iter()
                .map(|v| self[(rowidx, v)].to_string())
                .collect::<Vec<_>>()
                .join("");
            if let Some(&base) = rows.get(&rowkey) {
                // We have found a duplicate.
                if let Some(so) = &mut self.raw_energies {
                    if so[base] != so[rowidx] {
                        return Err(LunaModelError::Translation(
                            format!(
                            "the solution contains the same sample with different raw energies: for {base} is {}, for {rowidx} is {}", so[base], so[rowidx]).into(),
                        ));
                    }
                    // so.remove(rowidx);
                }
                if let Some(so) = &mut self.obj_values {
                    if so[base] != so[rowidx] {
                        return Err(LunaModelError::Translation(
                            format!(
                            "the solution contains the same sample with different objective values: for {base} is {}, for {rowidx} is {}", so[base], so[rowidx]).into(),
                        ));
                    }
                    // so.remove(rowidx);
                }
                if let Some(so) = &mut self.feasible {
                    if so[base] != so[rowidx] {
                        return Err(LunaModelError::Translation(
                            format!(
                            "the solution contains the same sample with different feasibilities: for {base} is {}, for {rowidx} is {}", so[base], so[rowidx]).into(),
                        ));
                    }
                    // so.remove(rowidx);
                }
                // We have checked the feasiblity, so we can assume the values in constraints and
                // variable bounds are equal, as the feasiblity is computed based on this info.
                // for (_, v) in self.constraints.iter_mut() {
                //     v.remove(rowidx);
                // }
                // for (_, v) in self.variable_bounds.iter_mut() {
                //     v.remove(rowidx);
                // }
                // self.counts[base] += self.counts[rowidx];
                to_rm.entry(base).or_insert(Vec::default()).push(rowidx);
            } else {
                rows.insert(rowkey, rowidx);
            }
        }

        for (&base, rmis) in to_rm.iter().rev() {
            for &rmi in rmis {
                self.counts[base] += self.counts[rmi];
                if let Some(e) = &mut self.raw_energies {
                    e.remove(rmi);
                }
                if let Some(e) = &mut self.obj_values {
                    e.remove(rmi);
                }
                if let Some(e) = &mut self.feasible {
                    e.remove(rmi);
                }
            }
        }
        for (_, v) in self.samples.iter_mut() {
            for (_, rmis) in to_rm.iter().rev() {
                for &rmi in rmis {
                    v.remove(rmi);
                }
            }
        }
        for (_, v) in self.constraints.iter_mut() {
            for (_, rmis) in to_rm.iter().rev() {
                for &rmi in rmis {
                    v.remove(rmi);
                }
            }
        }
        for (_, v) in self.variable_bounds.iter_mut() {
            for (_, rmis) in to_rm.iter().rev() {
                for &rmi in rmis {
                    v.remove(rmi);
                }
            }
        }

        Ok(())
    }
}
