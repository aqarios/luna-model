use std::rc::Rc;

use crate::core::{Sense, Solution};
use crate::serialization::Decodable;
use crate::{
    core::{solution::sol::SampleCol, RcSolution, Vtype},
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, DecodeError},
        utils::force_i8,
    },
};
use prost::Message;

fn u8_to_assignment_type(u: u8) -> Vtype {
    match u {
        0 => Vtype::Binary,
        1 => Vtype::Spin,
        2 => Vtype::Integer,
        3 => Vtype::Real,
        _ => panic!("issue"),
    }
}

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

/// Representation of encodable solution based on protocol buffers.
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
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerSolution {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerSolution conform with the requirements for it to be an Decodable.
impl BytesDecodable<RcSolution> for SerSolution {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<RcSolution, DecodeError> {
        Self::decode(bytes)?.extract()
    }
}

impl SerSolution {
    pub fn extract(&self) -> Result<RcSolution, DecodeError> {
        let mut sol = Solution::default();
        let num_samples = self.num_samples as usize;
        let mut type_per_pos: Vec<Vtype> = Vec::new();
        for &st in self.sample_types.iter() {
            let vt = u8_to_assignment_type(st);
            match vt {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(num_samples))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(num_samples))),
                Vtype::Integer => {
                    sol.add_column(SampleCol::Integer(Vec::with_capacity(num_samples)))
                }
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(num_samples))),
            }
            type_per_pos.push(vt);
        }

        sol.counts = self.counts.iter().map(|&v| v as usize).collect();
        sol.n_samples = self.num_samples as usize;

        let (mut lb, mut ls, mut li, mut lr) = (0, 0, 0, 0);
        let sample_len = self.sample_len as usize;
        for _ in 0..num_samples {
            for j in 0..sample_len {
                // let pos = i * num_samples + j;
                // println!("{pos}");
                match &type_per_pos[j] {
                    Vtype::Binary => {
                        sol.samples[j]
                            .push(self.bins[lb])
                            .expect("something went wrong");
                        lb += 1;
                    }
                    Vtype::Spin => {
                        sol.samples[j]
                            .push(force_i8(self.spins[ls]))
                            .expect("something went wrong");
                        ls += 1;
                    }
                    Vtype::Integer => {
                        sol.samples[j]
                            .push(self.ints[li])
                            .expect("something went wrong");
                        li += 1;
                    }
                    Vtype::Real => {
                        sol.samples[j]
                            .push(self.reals[lr])
                            .expect("something went wrong");
                        lr += 1;
                    }
                };
            }
        }
        sol.obj_values = vec![None; num_samples];
        if !self.obj_values.is_empty() {
            let mut idx = 0;
            for (i, &has_val) in self.has_obj_value.iter().enumerate() {
                if has_val {
                    sol.obj_values[i] = Some(self.obj_values[idx]);
                    idx += 1;
                }
            }
        }
        sol.raw_energies = vec![None; num_samples];
        if !self.raw_energies.is_empty() {
            let mut idx = 0;
            for (i, &has_val) in self.has_raw_energy.iter().enumerate() {
                if has_val {
                    sol.raw_energies[i] = Some(self.raw_energies[idx]);
                    idx += 1;
                }
            }
        }

        sol.best_sample_idx = self.best_sample_idx.map(|idx| idx as usize);

        if let Some(t) = &self.timing {
            sol.timing = Some(t.decode(())?);
        }

        sol.variable_names = self.variable_names.clone();

        sol.constraints = self
            .constraints
            .clone()
            .into_iter()
            .map(|item| item.vector.map(|v| v.values))
            .collect();

        sol.variable_bounds = self
            .variable_bounds
            .clone()
            .into_iter()
            .map(|item| item.vector.map(|v| v.values))
            .collect();

        sol.feasible = sol
            .constraints
            .iter()
            .zip(&sol.variable_bounds)
            .map(|x| match x {
                (None, _) => None,
                (_, None) => None,
                (Some(constr), Some(vbs)) => {
                    Some(constr.iter().all(|&b| b) && vbs.iter().all(|&b| b))
                }
            })
            .collect();

        sol.sense = match sol.best_sample_idx.and_then(|i| sol.obj_values[i]) {
            None => Sense::Min,
            Some(bobj) => {
                if sol
                    .obj_values
                    .iter()
                    .zip(&sol.feasible)
                    .all(|(&obj, &feas)| !feas.unwrap_or(true) || obj.unwrap_or(bobj) >= bobj)
                {
                    Sense::Min
                } else {
                    Sense::Max
                }
            }
        };

        Ok(RcSolution(Rc::new(sol)))
    }
}
