use std::str::FromStr;

use crate::{encode::{BytesDecodable, BytesEncodable, Decodable}, utils::u8_to_vtype};
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

        sol.n_samples = self.num_samples as usize;
        sol.samples = IndexMap::default();

        let (mut nbins, mut nspins, mut nints, mut nreals) = (0, 0, 0, 0);
        let sample_len = self.sample_len as usize;
        for (varname, st) in self.variable_names.iter().zip(self.sample_types) {
            let vtype = u8_to_vtype(st);
            if vtype.is_none() {
                continue;
            }
            match vtype.unwrap() {
                Vtype::Binary => {
                    let start_idx = nbins * sample_len;
                    let end_idx = start_idx + sample_len;
                    sol.add_binary(
                        varname.clone(),
                        self.bins[start_idx..end_idx]
                            .iter()
                            .map(|&e| e as f64)
                            .collect(),
                    );
                    nbins += 1;
                }
                Vtype::Spin => {
                    let start_idx = nspins * sample_len;
                    let end_idx = start_idx + sample_len;
                    sol.add_spin(
                        varname.clone(),
                        self.spins[start_idx..end_idx]
                            .iter()
                            .map(|&e| e as f64)
                            .collect(),
                    );
                    nspins += 1;
                }
                Vtype::Integer => {
                    let start_idx = nints * sample_len;
                    let end_idx = start_idx + sample_len;
                    sol.add_spin(
                        varname.clone(),
                        self.ints[start_idx..end_idx]
                            .iter()
                            .map(|&e| e as f64)
                            .collect(),
                    );
                    nints += 1;
                }
                Vtype::Real => {
                    let start_idx = nreals * sample_len;
                    let end_idx = start_idx + sample_len;
                    sol.add_real(
                        varname.clone(),
                        self.reals[start_idx..end_idx]
                            .iter()
                            .map(|&e| e as f64)
                            .collect(),
                    );
                    nreals += 1;
                }
                Vtype::InvertedBinary => (),
            };
        }

        sol.counts = self.counts.iter().map(|&c| c as usize).collect();
        if self.has_raw_energy.iter().all(|&b| b) {
            sol.raw_energies = Some(self.raw_energies);
        }
        if self.has_obj_value.iter().all(|&b| b) {
            sol.obj_values = Some(self.obj_values);
        }
        if self.constraints.iter().all(|vs| vs.vector.is_some()) {
            sol.constraints = self
                .constraints
                .into_iter()
                .map(|cs| {
                    if let Some(bs) = cs.vector {
                        bs.values
                            .iter()
                            .enumerate()
                            .map(|(i, c)| (format!("c{i}"), *c))
                            .collect::<HashMap<String, bool>>()
                    } else {
                        HashMap::default()
                    }
                })
                .collect();
        } else {
            sol.constraints = Vec::default();
        }
        if self.variable_bounds.iter().all(|vs| vs.vector.is_some()) {
            sol.variable_bounds = self
                .variable_bounds
                .into_iter()
                .zip(self.variable_names)
                .map(|(vbs, vn)| (vn, vbs.vector.unwrap().values))
                .collect();
        } else {
            sol.variable_bounds = HashMap::default();
        }
        let mut feasible = vec![true; sol.n_samples];
        sol.constraints
            .iter()
            .enumerate()
            .for_each(|(i, map)| feasible[i] = feasible[i] && map.iter().all(|(_, b)| *b));
        sol.variable_bounds.iter().for_each(|(_, bs)| {
            bs.iter()
                .enumerate()
                .for_each(|(si, b)| feasible[si] = feasible[si] && *b)
        });
        sol.feasible = Some(feasible);
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
