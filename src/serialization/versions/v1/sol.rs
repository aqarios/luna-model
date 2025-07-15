use bitvec::vec::BitVec;
use bitvec::view::AsBits;
use prost::Message;

use crate::core::solution::Column;
use crate::serialization::encodable::{Creatable, DecodeError};
use crate::serialization::Encodable;
use crate::{
    core::Solution,
    serialization::encodable::{BytesDecodable, BytesEncodable},
};

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

    #[prost(bytes, tag = 5)]
    bins: Vec<u8>,
    #[prost(bytes, tag = 6)]
    spins: Vec<u8>,
    #[prost(int64, repeated, tag = 7)]
    ints: Vec<i64>,
    #[prost(double, repeated, tag = 8)]
    reals: Vec<f64>,

    /// The number of occurrences for each sample in the solution.
    #[prost(uint64, repeated, tag = 9)]
    counts: Vec<u64>,
    /// The objective value for each sample in the solution
    #[prost(double, repeated, tag = 10)]
    obj_values: Vec<f64>, // inherently optional
    /// The raw energies for each sample in the solution
    #[prost(double, repeated, tag = 11)]
    raw_energies: Vec<f64>, // inherently optional
    /// The index of the best sample
    #[prost(uint64, optional, tag = 13)]
    best_sample_idx: Option<u64>,

    #[prost(bytes, tag = 16)]
    constraints: Vec<u8>,
    #[prost(bytes, tag = 17)]
    variable_bounds: Vec<u8>,
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
        self.obj_values = sol.obj_values.map_or_else(Vec::default, |o| o.clone());
        self.raw_energies = sol.raw_energies.map_or_else(Vec::default, |r| r.clone());

        for col in sol.samples.iter() {
            match &col {
                Column::Binary(inner) => self
                    .bins
                    .extend(inner.data.as_bits().to_bitvec().into_vec()),
                Column::Spin(inner) => self.spins.extend(
                    inner
                        .data
                        .iter()
                        .map(|&s| s == 1)
                        .collect()
                        .as_bits()
                        .to_bitvec()
                        .into_vec(),
                ),
                Column::Integer(inner) => self.ints.extend(inner.data.clone()),
                Column::Real(inner) => self.reals.extend(inner.data.clone()),
            }

            dbg!(&self.bins, &self.bins.len());
            dbg!(&self.spins, &self.spins.len());
            dbg!(&self.ints, &self.ints.len());
            dbg!(&self.reals, &self.reals.len());
        }

        self.constraints = sol.constraints.map_or_else(Vec::default, |cs| {
            let mut flat = Vec::default();
            for cv in cs.iter() {
                flat.extend(cv.iter().collect().as_bits().to_bitvec().into_vec());
            }
            flat
        });
        self.variable_bounds = sol.variable_bounds.map_or_else(Vec::default, |cs| {
            let mut flat = Vec::default();
            for cv in cs.iter() {
                flat.extend(cv.iter().collect().as_bits().to_bitvec().into_vec());
            }
            flat
        });
        self
    }

    fn extract(&self) -> Result<Solution, DecodeError> {
        todo!()
    }
}
