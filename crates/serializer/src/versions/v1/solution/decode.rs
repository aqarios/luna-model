use std::str::FromStr;

use bitvec::{order::Lsb0, vec::BitVec};
use lunamodel_core::Solution;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Sense, Vtype};
use prost::Message;

use crate::{
    encode::{BytesDecodable, Decodable},
    utils::u8_to_vtype,
};

use super::SerSolution;

/// Makes the SerSolution conform with the requirements for it to be an Decodable.
impl BytesDecodable<Solution> for SerSolution {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<Solution> {
        Self::decode(bytes)?.extract()
    }
}

impl SerSolution {
    fn extract(self) -> LunaModelResult<Solution> {
        let mut sol = Solution::with_sense(
            Sense::from_str(&self.sense)
                .map_err(|e| LunaModelError::Decoding(e.to_string().into()))?,
        );
        sol.counts = self.counts.iter().map(|&c| c as usize).collect();
        sol.obj_values = match self.obj_values.is_empty() {
            true => None,
            false => Some(self.obj_values),
        };
        sol.raw_energies = match self.raw_energies.is_empty() {
            true => None,
            false => Some(self.raw_energies),
        };

        if let Some(t) = self.timing {
            sol.timing = Some(t.decode(())?);
        }

        let mut bv: BitVec<u8, Lsb0> = BitVec::from_vec(self.bins);
        bv.truncate(self.num_samples as usize * self.n_bins as usize);
        let bins: Vec<u8> = bv.into_iter().map(|v| v as u8).collect();

        let mut sv: BitVec<u8, Lsb0> = BitVec::from_vec(self.spins);
        sv.truncate(self.num_samples as usize * self.n_spins as usize);
        let spins: Vec<i8> = sv.into_iter().map(|v| 1 - (2 * v as i8)).collect();

        let (mut start_bin, mut start_spin, mut start_int, mut start_real) = (0, 0, 0, 0);
        for (name, &st) in self
            .variable_names
            .iter()
            .cloned()
            .zip(self.sample_types.iter())
        {
            let vtype = u8_to_vtype(st);
            if vtype.is_none() {
                continue;
            }
            match vtype.unwrap() {
                Vtype::Binary => {
                    sol.add_binary(
                        name,
                        bins[start_bin..start_bin + self.num_samples as usize]
                            .iter()
                            .map(|&v| v as f64)
                            .collect(),
                        None,
                    )?;
                    start_bin += self.num_samples as usize;
                }
                Vtype::Spin => {
                    sol.add_spin(
                        name,
                        spins[start_spin..start_spin + self.num_samples as usize]
                            .iter()
                            .map(|&v| v as f64)
                            .collect(),
                        None,
                    )?;
                    start_spin += self.num_samples as usize;
                }
                Vtype::Integer => {
                    sol.add_integer(
                        name,
                        self.ints[start_int..start_int + self.num_samples as usize]
                            .iter()
                            .map(|&v| v as f64)
                            .collect(),
                        None,
                    )?;
                    start_int += self.num_samples as usize;
                }
                Vtype::Real => {
                    sol.add_real(
                        name,
                        self.reals[start_real..start_real + self.num_samples as usize].to_vec(),
                    );
                    start_real += self.num_samples as usize;
                }
                Vtype::InvertedBinary => (),
            }
        }

        let constraint_names = if self.constraint_names.is_empty() {
            (0..self.n_constraints).map(|i| format!("c{i}")).collect()
        } else {
            self.constraint_names
        };

        let mut cv: BitVec<u8, Lsb0> = BitVec::from_vec(self.constraints);
        cv.truncate(self.n_constraints as usize * self.num_samples as usize);
        let constraint_chunks: Vec<Vec<bool>> = match self.n_constraints == 0 {
            true => Vec::default(),
            false => cv
                .into_iter()
                .collect::<Vec<_>>()
                .chunks_exact(self.n_constraints as usize)
                .map(|chunk| chunk.to_vec())
                .collect(),
        };

        for sample in constraint_chunks {
            for (cname, value) in constraint_names.iter().zip(sample) {
                sol.constraints
                    .entry(cname.clone())
                    .or_default()
                    .push(value);
            }
        }

        let mut cv: BitVec<u8, Lsb0> = BitVec::from_vec(self.variable_bounds);
        cv.truncate(self.n_variable_bounds as usize * self.num_samples as usize);
        let variable_bounds: Vec<Vec<bool>> = match self.n_variable_bounds == 0 {
            true => Vec::default(),
            false => cv
                .into_iter()
                .collect::<Vec<_>>()
                .chunks_exact(self.n_variable_bounds as usize)
                .map(|chunk| chunk.to_vec())
                .collect(),
        };
        let variable_bound_names = if self.variable_bound_names.is_empty() {
            self.variable_names
        } else {
            self.variable_bound_names
        };
        for sample in variable_bounds {
            for (vname, value) in variable_bound_names.iter().zip(sample) {
                sol.variable_bounds
                    .entry(vname.clone())
                    .or_default()
                    .push(value);
            }
        }

        sol.feasible = if sol.variable_bounds.is_empty() && sol.constraints.is_empty() {
            None
        } else {
            let mut feasible: Vec<bool> = vec![true; self.num_samples as usize];
            for per_constraint in sol.constraints.values() {
                for (i, &item) in per_constraint.iter().enumerate() {
                    feasible[i] = feasible[i] && item;
                }
            }
            for per_sample in sol.variable_bounds.values() {
                for (i, &item) in per_sample.iter().enumerate() {
                    feasible[i] = feasible[i] && item;
                }
            }
            Some(feasible)
        };

        Ok(sol)
    }
}
