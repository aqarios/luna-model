use std::str::FromStr;
use std::usize;

use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use prost::Message;

use crate::core::solution::Column;
use crate::core::{Sense, Vtype};
use crate::serialization::encodable::{Creatable, DecodeError};
use crate::serialization::{Decodable, Encodable};
use crate::{
    core::Solution,
    serialization::encodable::{BytesDecodable, BytesEncodable},
};

fn vtype_to_u8(vtype: Vtype) -> u8 {
    match vtype {
        Vtype::Binary => 0,
        Vtype::Spin => 1,
        Vtype::Integer => 2,
        Vtype::Real => 3,
        Vtype::__Ghost => 4,
        Vtype::InvertedBinary => 5,
    }
}

fn u8_to_vtype(u: u8) -> Vtype {
    match u {
        0 => Vtype::Binary,
        1 => Vtype::Spin,
        2 => Vtype::Integer,
        3 => Vtype::Real,
        4 => Vtype::__Ghost,
        5 => Vtype::InvertedBinary,
        _ => unreachable!("issue"),
    }
}

type Bv = u8;

/// Representation of encodable solution based on protocol buffers.
#[derive(Clone, PartialEq, Message)]
pub struct SerSolution {
    /// The number of samples in the solution.
    #[prost(uint64, tag = 1)]
    num_samples: u64,
    /// The variable names
    #[prost(string, repeated, tag = 2)]
    variable_names: Vec<String>,
    /// The sense for which this solution was created
    #[prost(string, tag = 3)]
    sense: String,
    /// Runtime metrics of the solution.
    #[prost(bytes, optional, tag = 4)]
    timing: Option<Vec<u8>>,
    /// The type for each element in a sample
    #[prost(bytes, tag = 5)]
    sample_types: Vec<u8>,

    #[prost(bytes, tag = 6)]
    bins: Vec<Bv>,
    #[prost(uint64, tag = 7)]
    n_bins: u64,

    #[prost(bytes, tag = 8)]
    spins: Vec<Bv>,
    #[prost(uint64, tag = 9)]
    n_spins: u64,

    #[prost(int64, repeated, tag = 10)]
    ints: Vec<i64>,

    #[prost(double, repeated, tag = 11)]
    reals: Vec<f64>,

    /// The number of occurrences for each sample in the solution.
    #[prost(uint64, repeated, tag = 12)]
    counts: Vec<u64>,
    /// The objective value for each sample in the solution
    #[prost(double, repeated, tag = 13)]
    obj_values: Vec<f64>, // inherently optional
    /// The raw energies for each sample in the solution
    #[prost(double, repeated, tag = 14)]
    raw_energies: Vec<f64>, // inherently optional
    /// The index of the best sample
    #[prost(uint64, optional, tag = 16)]
    best_sample_idx: Option<u64>,

    #[prost(bytes, tag = 17)]
    constraints: Vec<u8>,
    #[prost(uint64, tag = 18)]
    n_constraints: u64,

    #[prost(bytes, tag = 19)]
    variable_bounds: Vec<u8>,
    #[prost(uint64, tag = 20)]
    n_variable_bounds: u64,
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerSolution {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerSolution conform with the requirements for it to be an Decodable.
impl BytesDecodable<Solution> for SerSolution {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<Solution, DecodeError> {
        Self::decode(bytes)?.extract()
    }
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl Creatable<Solution> for SerSolution {
    fn new(value: &Solution) -> Self {
        Self::default().fill(&value)
    }
}

impl SerSolution {
    fn fill(mut self, sol: &Solution) -> Self {
        self.num_samples = sol.len() as u64;
        // todo: maybe improve this.
        self.variable_names = sol.variable_names.clone();
        self.sense = sol.sense.to_string();
        self.timing = sol.timing.map(|t| t.serialize());
        self.best_sample_idx = sol.best_sample_idx.map(|b| b as u64);
        self.counts = sol.counts.iter().map(|&c| c as u64).collect();
        self.obj_values = sol.obj_values.clone().unwrap_or_else(Vec::default);
        self.raw_energies = sol.raw_energies.clone().unwrap_or_else(Vec::default);

        let mut bin_vec: BitVec<Bv, Lsb0> = BitVec::new();
        let mut spin_vec: BitVec<Bv, Lsb0> = BitVec::new();

        for col in sol.samples.iter() {
            match &col {
                Column::Binary(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Binary));
                    self.n_bins += 1;
                    bin_vec.extend(inner.data.iter().map(|&b| b == 1).into_iter())
                }
                Column::Spin(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Spin));
                    self.n_spins += 1;
                    spin_vec.extend(inner.data.iter().map(|&s| s == -1).into_iter())
                }
                Column::Integer(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Integer));
                    self.ints.extend(inner.data.clone())
                }
                Column::Real(inner) => {
                    self.sample_types.push(vtype_to_u8(Vtype::Real));
                    self.reals.extend(inner.data.clone())
                }
            }
        }

        self.bins = bin_vec.into_vec();
        self.spins = spin_vec.into_vec();

        // self.n_constraints = sol.constraints.as_ref().map_or(0, |c| c.len() as u64);
        self.constraints = sol.constraints.clone().map_or_else(Vec::default, |cs| {
            let mut flat: Vec<bool> = Vec::default();
            for cv in cs.iter() {
                // not sure if constraints is not none. here we are sure.
                // so update is done multiple times for now...
                self.n_constraints = cv.len() as u64;
                flat.extend(cv);
            }
            flat.into_iter().collect::<BitVec<u8, Lsb0>>().into_vec()
        });
        self.variable_bounds = sol.variable_bounds.clone().map_or_else(Vec::default, |cs| {
            let mut flat: Vec<bool> = Vec::default();
            for cv in cs.iter() {
                // not sure if variable_bounds is not none. here we are sure.
                // so update is done multiple times for now...
                self.n_variable_bounds = cv.len() as u64;
                flat.extend(cv);
            }
            flat.into_iter().collect::<BitVec<u8, Lsb0>>().into_vec()
        });
        self
    }

    fn extract(self) -> Result<Solution, DecodeError> {
        let mut sol = Solution::with_sense(
            Sense::from_str(&self.sense).map_err(|e| DecodeError::new(e.to_string()))?,
        );
        sol.n_samples = self.num_samples as usize;
        sol.variable_names = self.variable_names;
        sol.best_sample_idx = self.best_sample_idx.map(|i| i as usize);
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

        let mut bv: BitVec<Bv, Lsb0> = BitVec::from_vec(self.bins);
        bv.truncate(sol.n_samples * self.n_bins as usize);
        let bins: Vec<u8> = bv.into_iter().map(|v| v as u8).collect();

        let mut sv: BitVec<Bv, Lsb0> = BitVec::from_vec(self.spins);
        sv.truncate(sol.n_samples * self.n_spins as usize);
        let spins: Vec<i8> = sv.into_iter().map(|v| 1 - (2 * v as i8)).collect();

        let (mut start_bin, mut start_spin, mut start_int, mut start_real) = (0, 0, 0, 0);
        for (i, &st) in self.sample_types.iter().enumerate() {
            let vtype = u8_to_vtype(st);
            match vtype {
                Vtype::__Ghost => (),
                Vtype::InvertedBinary => (),
                Vtype::Binary => {
                    sol.add_binary_col(
                        i.into(),
                        bins[start_bin..start_bin + sol.n_samples].to_vec(),
                    );
                    start_bin += sol.n_samples;
                }
                Vtype::Spin => {
                    sol.add_spin_col(
                        i.into(),
                        spins[start_spin..start_spin + sol.n_samples].to_vec(),
                    );
                    start_spin += sol.n_samples;
                }
                Vtype::Integer => {
                    sol.add_integer_col(
                        i.into(),
                        self.ints[start_int..start_int + sol.n_samples].to_vec(),
                    );
                    start_int += sol.n_samples;
                }
                Vtype::Real => {
                    sol.add_real_col(
                        i.into(),
                        self.reals[start_real..start_real + sol.n_samples].to_vec(),
                    );
                    start_real += sol.n_samples;
                }
            }
        }

        sol.constraints = match self.n_constraints == 0 {
            true => None,
            false => Some({
                let mut cv: BitVec<u8, Lsb0> = BitVec::from_vec(self.constraints);
                cv.truncate(self.n_constraints as usize * sol.n_samples);
                cv.into_iter()
                    .collect::<Vec<_>>()
                    .chunks_exact(self.n_constraints as usize)
                    .map(|chunk| chunk.to_vec())
                    .collect()
            }),
        };
        sol.variable_bounds = match self.n_variable_bounds == 0 {
            true => None,
            false => Some({
                let mut cv: BitVec<u8, Lsb0> = BitVec::from_vec(self.variable_bounds);
                cv.truncate(self.n_variable_bounds as usize * sol.n_samples);
                cv.into_iter()
                    .collect::<Vec<_>>()
                    .chunks_exact(self.n_variable_bounds as usize)
                    .map(|chunk| chunk.to_vec())
                    .collect()
            }),
        };

        sol.feasible = match (&sol.constraints, &sol.variable_bounds) {
            (Some(c), Some(b)) => Some(
                c.iter()
                    .zip(b)
                    .map(|(c, v)| c.iter().all(|&b| b) && v.iter().all(|&b| b))
                    .collect(),
            ),
            (Some(c), None) => Some(c.iter().map(|c| c.iter().all(|&b| b)).collect()),
            (None, Some(b)) => Some(b.iter().map(|b| b.iter().all(|&b| b)).collect()),
            (None, None) => None,
        };

        Ok(sol)
    }
}
