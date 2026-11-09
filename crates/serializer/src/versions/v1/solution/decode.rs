//! Version 1 decoding for solutions.

use std::str::FromStr;

use bitvec::{order::Lsb0, vec::BitVec};
use lunamodel_core::Solution;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Sense, Vtype};
use prost::Message;

use crate::{
    check::{aligned, product, span},
    encode::{BytesDecodable, Decodable},
    utils::u8_to_vtype,
    versionize::Unversionizable,
};

use super::SerSolution;

/// Makes the SerSolution conform with the requirements for it to be an Decodable.
impl BytesDecodable<Solution> for SerSolution {
    /// Decodes version-1 bytes into a solution.
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<Solution> {
        Self::decode(bytes)?.extract()
    }
}

impl SerSolution {
    /// Extracts the runtime solution from the protobuf structure.
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
            sol.timing = Some(t.as_slice().unversionize_nested().decode(())?);
        }

        let num_samples = self.num_samples as usize;
        let mut bv: BitVec<u8, Lsb0> = BitVec::from_vec(self.bins);
        bv.truncate(product(self.num_samples, self.n_bins, "binary samples")?);
        let bins: Vec<u8> = bv.into_iter().map(|v| v as u8).collect();

        let mut sv: BitVec<u8, Lsb0> = BitVec::from_vec(self.spins);
        sv.truncate(product(self.num_samples, self.n_spins, "spin samples")?);
        let spins: Vec<i8> = sv.into_iter().map(|v| 1 - (2 * v as i8)).collect();

        let (mut start_bin, mut start_spin, mut start_int, mut start_real) = (0, 0, 0, 0);
        for (name, &st) in self
            .variable_names
            .iter()
            .cloned()
            .zip(self.sample_types.iter())
        {
            let Some(vtype) = u8_to_vtype(st)? else {
                continue;
            };
            match vtype {
                Vtype::Binary => {
                    sol.add_binary(
                        name,
                        span(&bins, start_bin, num_samples, "binary samples")?
                            .iter()
                            .map(|&v| v as f64)
                            .collect(),
                        None,
                    )?;
                    start_bin += num_samples;
                }
                Vtype::Spin => {
                    sol.add_spin(
                        name,
                        span(&spins, start_spin, num_samples, "spin samples")?
                            .iter()
                            .map(|&v| v as f64)
                            .collect(),
                        None,
                    )?;
                    start_spin += num_samples;
                }
                Vtype::Integer => {
                    sol.add_integer(
                        name,
                        span(&self.ints, start_int, num_samples, "integer samples")?
                            .iter()
                            .map(|&v| v as f64)
                            .collect(),
                        None,
                    )?;
                    start_int += num_samples;
                }
                Vtype::Real => {
                    sol.add_real(
                        name,
                        span(&self.reals, start_real, num_samples, "real samples")?.to_vec(),
                    );
                    start_real += num_samples;
                }
                Vtype::InvertedBinary => (),
            }
        }

        let mut cv: BitVec<u8, Lsb0> = BitVec::from_vec(self.constraints);
        cv.truncate(product(
            self.n_constraints,
            self.num_samples,
            "constraints",
        )?);
        let constraint_chunks: Vec<Vec<bool>> = match self.n_constraints == 0 {
            true => Vec::default(),
            false => cv
                .into_iter()
                .collect::<Vec<_>>()
                .chunks_exact(self.n_constraints as usize)
                .map(|chunk| chunk.to_vec())
                .collect(),
        };

        // Generated only for existing results, so `n_constraints` is bounded by the data.
        let constraint_names = match (
            self.constraint_names.is_empty(),
            constraint_chunks.is_empty(),
        ) {
            (true, false) => (0..self.n_constraints).map(|i| format!("c{i}")).collect(),
            _ => self.constraint_names,
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
        cv.truncate(product(
            self.n_variable_bounds,
            self.num_samples,
            "variable bounds",
        )?);
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

        aligned(&sol)?;
        sol.feasible = if sol.variable_bounds.is_empty() && sol.constraints.is_empty() {
            None
        } else {
            let mut feasible: Vec<bool> = vec![true; sol.len()];
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
