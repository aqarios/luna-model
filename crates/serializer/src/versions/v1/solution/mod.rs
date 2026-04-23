mod decode;
mod encode;

use prost::Message;

use lunamodel_core::Solution;

use crate::encode::Creatable;

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
    bins: Vec<u8>,
    #[prost(uint64, tag = 7)]
    n_bins: u64,

    #[prost(bytes, tag = 8)]
    spins: Vec<u8>,
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

    // /// The index of the best sample
    // #[prost(uint64, optional, tag = 16)]
    // best_sample_idx: Option<u64>,
    #[prost(bytes, tag = 17)]
    constraints: Vec<u8>,
    #[prost(uint64, tag = 18)]
    n_constraints: u64,
    #[prost(string, repeated, tag = 21)]
    constraint_names: Vec<String>,

    #[prost(bytes, tag = 19)]
    variable_bounds: Vec<u8>,
    #[prost(uint64, tag = 20)]
    n_variable_bounds: u64,
    #[prost(string, repeated, tag = 22)]
    variable_bound_names: Vec<String>,
}

/// Makes the SerSolution conform with the requirements for it to be an Encodable.
impl Creatable<Solution> for SerSolution {
    fn new(value: &Solution) -> Self {
        Self::default().fill(value)
    }
}
