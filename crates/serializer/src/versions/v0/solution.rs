use std::str::FromStr;

use crate::{
    encode::{BytesDecodable, BytesEncodable, Decodable},
    utils::u8_to_vtype,
};
use hashbrown::HashMap;
use indexmap::IndexMap;
use lunamodel_core::Solution;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Sense, Vtype};
use prost::Message;

#[derive(Clone, PartialEq, Message)]
struct BoolVec {
    #[prost(bool, repeated, tag = "1")]
    values: Vec<bool>,
}

#[derive(Clone, PartialEq, Message)]
struct OptBoolVec {
    #[prost(message, optional, tag = "1")]
    vector: Option<BoolVec>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SerSolution {
    /// The number of samples in the solution.
    #[prost(uint64, tag = 1)]
    num_samples: u64,

    /// The length of each sample
    #[prost(uint32, tag = 2)]
    sample_len: u32,
    /// The type for each element in a sample
    #[prost(bytes, tag = 3)]
    sample_types: Vec<u8>,

    #[prost(bytes, tag = 4)]
    bins: Vec<u8>,

    #[prost(int32, repeated, tag = 5)]
    spins: Vec<i32>,

    #[prost(int64, repeated, tag = 6)]
    ints: Vec<i64>,

    #[prost(double, repeated, tag = 7)]
    reals: Vec<f64>,

    /// The number of occurrences for each sample in the solution.
    #[prost(uint64, repeated, tag = 8)]
    counts: Vec<u64>,

    /// The objective value for each sample in the solution
    #[prost(double, repeated, tag = 9)]
    obj_values: Vec<f64>,

    /// If a sample has an objective value stored. Length corresponds to
    /// num_samples.
    #[prost(bool, repeated, tag = 10)]
    has_obj_value: Vec<bool>,

    /// The raw energies for each sample in the solution
    #[prost(double, repeated, tag = 11)]
    raw_energies: Vec<f64>,

    /// If a sample has a raw energy stored. Length corresponds to
    /// num_samples.
    #[prost(bool, repeated, tag = 12)]
    has_raw_energy: Vec<bool>,

    /// The index of the best sample
    #[prost(uint64, optional, tag = 13)]
    best_sample_idx: Option<u64>,

    /// Runtime metrics of the solution.
    #[prost(bytes, optional, tag = 14)]
    timing: Option<Vec<u8>>,

    /// The variable names
    #[prost(string, repeated, tag = 15)]
    variable_names: Vec<String>,

    #[prost(message, repeated, tag = 16)]
    constraints: Vec<OptBoolVec>,

    #[prost(message, repeated, tag = 17)]
    variable_bounds: Vec<OptBoolVec>,

    #[prost(string, optional, tag = 18)]
    sense: Option<String>,
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerSolution {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerSolution conform with the requirements for it to be an Decodable.
impl BytesDecodable<Solution> for SerSolution {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<Solution> {
        Self::decode(bytes)?.extract()
    }
}

impl SerSolution {
    pub fn extract(self) -> LunaModelResult<Solution> {
        let mut sol = Solution::default();

        sol.samples = IndexMap::default();

        let (mut nbins, mut nspins, mut nints, mut nreals) = (0, 0, 0, 0);
        let num_samples = self.num_samples as usize;

        if num_samples != 0 {
            let bin_step: usize = self.bins.len() / num_samples;
            let spin_step: usize = self.spins.len() / num_samples;
            let int_step: usize = self.ints.len() / num_samples;
            let real_step: usize = self.reals.len() / num_samples;

            for (varname, st) in self.variable_names.iter().zip(self.sample_types) {
                let vtype = u8_to_vtype(st);
                if vtype.is_none() {
                    continue;
                }
                match vtype.unwrap() {
                    Vtype::Binary => {
                        sol.add_binary(
                            varname.clone(),
                            self.bins
                                .iter()
                                .skip(nbins)
                                .step_by(bin_step)
                                .map(|&e| e as f64)
                                .collect(),
                        );
                        nbins += 1;
                    }
                    Vtype::Spin => {
                        sol.add_spin(
                            varname.clone(),
                            self.spins // [start_idx..end_idx]
                                .iter()
                                .skip(nspins)
                                .step_by(spin_step)
                                .map(|&e| e as f64)
                                .collect(),
                        );
                        nspins += 1;
                    }
                    Vtype::Integer => {
                        sol.add_integer(
                            varname.clone(),
                            self.ints
                                .iter()
                                .skip(nints)
                                .step_by(int_step)
                                .map(|&e| e as f64)
                                .collect(),
                        );
                        nints += 1;
                    }
                    Vtype::Real => {
                        sol.add_real(
                            varname.clone(),
                            self.reals
                                .iter()
                                .skip(nreals)
                                .step_by(real_step)
                                .map(|&e| e as f64)
                                .collect(),
                        );
                        nreals += 1;
                    }
                    Vtype::InvertedBinary => (),
                };
            }
        }

        sol.counts = self.counts.iter().map(|&c| c as usize).collect();
        if self.has_raw_energy.iter().all(|&b| b) {
            sol.raw_energies = Some(self.raw_energies);
        }
        if self.has_obj_value.iter().all(|&b| b) {
            sol.obj_values = Some(self.obj_values);
        }
        if !self.constraints.is_empty() && self.constraints.iter().all(|vs| vs.vector.is_some()) {
            // outer is samples, inner is values.
            // constraints[i] -> samples[i]
            // constraints[i][constraint]
            let len: usize = self.constraints[0].vector.as_ref().unwrap().values.len();
            let cnames: Vec<_> = (0..len).map(|i| format!("c{i}")).collect();
            for sample in self.constraints {
                for (cname, value) in cnames.iter().zip(sample.vector.unwrap().values) {
                    sol.constraints
                        .entry(cname.to_string())
                        .or_insert(Vec::default())
                        .push(value);
                    // sol.constraints.get_mut(cname).unwrap().push(value);
                }
            }
        } else {
            sol.constraints = HashMap::default();
        }
        if !self.variable_bounds.is_empty()
            && self.variable_bounds.iter().all(|vs| vs.vector.is_some())
        {
            // outer is samples, inner is values.
            // variable_bounds[i] -> samples[i]
            // variable_bounds[i][var] -> samples[i][var]
            sol.variable_bounds = self
                .variable_names
                .iter()
                .map(|v| (v.clone(), Vec::default()))
                .collect();

            for sample in self.variable_bounds {
                for (varname, value) in self
                    .variable_names
                    .iter()
                    .zip(sample.vector.unwrap().values)
                {
                    sol.variable_bounds.get_mut(varname).unwrap().push(value);
                }
            }
        } else {
            sol.variable_bounds = HashMap::default();
        }
        if !(sol.constraints.is_empty() && sol.variable_bounds.is_empty()) {
            let mut feasible = vec![true; sol.n_samples()];
            sol.constraints.iter().for_each(|(_, bs)| {
                bs.iter()
                    .enumerate()
                    .for_each(|(si, b)| feasible[si] = feasible[si] && *b)
            });
            sol.variable_bounds.iter().for_each(|(_, bs)| {
                bs.iter()
                    .enumerate()
                    .for_each(|(si, b)| feasible[si] = feasible[si] && *b)
            });
            sol.feasible = Some(feasible);
        }
        if let Some(t) = self.timing {
            sol.timing = Some(t.decode(())?);
        }

        match self.sense {
            Some(sense) => {
                let sense = Sense::from_str(&sense)
                    .map_err(|e| LunaModelError::Decoding(e.to_string().into()))?;
                sol.sense = sense;
            }
            None => {
                let bestobj = self
                    .best_sample_idx
                    .and_then(|i| sol.obj_values.as_ref().map(|o| o[i as usize]));
                sol.sense = match bestobj {
                    None => Sense::Min,
                    Some(b) => match (sol.obj_values.as_ref(), sol.feasible.as_ref()) {
                        (Some(o), Some(f)) => {
                            if o.iter().zip(f).all(|(&obj, &feas)| !feas || obj >= b) {
                                Sense::Min
                            } else {
                                Sense::Max
                            }
                        }
                        _ => Sense::Min,
                    },
                }
            }
        }

        Ok(sol)
    }
}
