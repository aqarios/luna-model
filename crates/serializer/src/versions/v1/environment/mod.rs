mod decode;
mod encode;

use lunamodel_core::prelude::Environment;
use prost::Message;

use crate::encode::Creatable;

#[derive(Clone, PartialEq, Message)]
pub struct SerEnvironment {
    // NOTE: Old "id" field has been removed
    // id: u32, // tag = 1 (Removed)
    /// The number of variables registered in the environment.
    #[prost(uint32, tag = "2")]
    varcount: u32,
    /// The length of the variables vector.
    #[prost(uint32, optional, tag = "20")]
    next_idx: Option<u32>,

    /// The indices of the binary variables.
    #[prost(uint32, repeated, tag = "3")]
    binary: Vec<u32>,
    /// The indices of the binary variables.
    #[prost(bool, repeated, tag = "23")]
    binary_is_inverted: Vec<bool>,
    /// The indices of the inverted binary variables.
    #[prost(uint32, repeated, tag = "21")]
    inverted_binary: Vec<u32>,
    /// The indices of the spin variables.
    #[prost(uint32, repeated, tag = "4")]
    spin: Vec<u32>,
    /// The indices of the integer variables.
    #[prost(uint32, repeated, tag = "5")]
    integer: Vec<u32>,
    /// The indices of the real variables.
    #[prost(uint32, repeated, tag = "6")]
    real: Vec<u32>,

    /// The names of the binary variables
    #[prost(string, repeated, tag = "7")]
    binary_names: Vec<String>,
    /// The names of the spin variables
    #[prost(string, repeated, tag = "8")]
    spin_names: Vec<String>,
    /// The names of the integer variables
    #[prost(string, repeated, tag = "9")]
    integer_names: Vec<String>,
    /// The names of the real variables
    #[prost(string, repeated, tag = "10")]
    real_names: Vec<String>,

    /// If the integer at each index has a non-default bound.
    #[prost(bool, repeated, tag = "11")]
    integer_bounds_has_lower: Vec<bool>,
    /// If the integer at each index has a non-default bound.
    #[prost(bool, repeated, tag = "12")]
    integer_bounds_has_upper: Vec<bool>,
    /// The integer lower bounds
    #[prost(double, repeated, tag = "13")]
    integer_bounds_lower: Vec<f64>,
    /// The integer upper bounds
    #[prost(double, repeated, tag = "14")]
    integer_bounds_upper: Vec<f64>,

    /// If the real at each index has a non-default bound.
    #[prost(bool, repeated, tag = "15")]
    real_bounds_has_lower: Vec<bool>,
    /// If the real at each index has a non-default bound.
    #[prost(bool, repeated, tag = "16")]
    real_bounds_has_upper: Vec<bool>,
    /// The reals' lower bounds.
    #[prost(double, repeated, tag = "17")]
    real_bounds_lower: Vec<f64>,
    /// The reals' upper bounds.
    #[prost(double, repeated, tag = "18")]
    real_bounds_upper: Vec<f64>,
}

impl Creatable<Environment> for SerEnvironment {
    fn new(value: &Environment) -> Self {
        Self::default().fill(&value)
    }
}
