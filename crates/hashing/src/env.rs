//! Hash encoding for environments.

use lunamodel_core::ArcEnv;
use lunamodel_types::{Bound, Vtype};
use prost::Message;

/// Representation of a bytes encodable/decodable environment.
#[derive(Clone, PartialEq, Message)]
pub struct HashEnv {
    /// The environment id.
    #[prost(uint32, tag = "1")]
    id: u32,
    /// The number of variables registered in the environment.
    #[prost(uint32, tag = "2")]
    varcount: u32,

    /// The indices of the binary variables.
    #[prost(uint32, repeated, tag = "3")]
    binary: Vec<u32>,
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
    integer_bounds_bounded_lower: Vec<bool>,
    /// If the integer at each index has a non-default bound.
    #[prost(bool, repeated, tag = "12")]
    integer_bounds_bounded_upper: Vec<bool>,
    /// The integer lower bounds
    #[prost(double, repeated, tag = "13")]
    integer_bounds_lower: Vec<f64>,
    /// The integer upper bounds
    #[prost(double, repeated, tag = "14")]
    integer_bounds_upper: Vec<f64>,

    /// If the real at each index has a non-default bound.
    #[prost(bool, repeated, tag = "15")]
    real_bounds_bounded_lower: Vec<bool>,
    /// If the real at each index has a non-default bound.
    #[prost(bool, repeated, tag = "16")]
    real_bounds_bounded_upper: Vec<bool>,
    /// The reals' lower bounds.
    #[prost(double, repeated, tag = "17")]
    real_bounds_lower: Vec<f64>,
    /// The reals' upper bounds.
    #[prost(double, repeated, tag = "18")]
    real_bounds_upper: Vec<f64>,
}

impl HashEnv {
    /// Encodes an environment into the hashing representation.
    ///
    /// Variables are traversed in index order so the output is deterministic for
    /// a fixed environment content.
    pub fn build(env: &ArcEnv) -> Vec<u8> {
        let mut e = HashEnv::default();

        let mut sorted_vars = env.vars();
        sorted_vars.sort_by_key(|a| a.id());

        for (i, var) in sorted_vars.iter().enumerate() {
            e.varcount += 1;
            let vname = var.name().unwrap();
            match var.vtype().unwrap() {
                Vtype::InvertedBinary => (),
                Vtype::Binary => {
                    e.binary.push(i as u32);
                    e.binary_names.push(vname);
                }
                Vtype::Spin => {
                    e.spin.push(i as u32);
                    e.spin_names.push(vname);
                }
                Vtype::Integer => {
                    e.integer.push(i as u32);
                    e.integer_names.push(vname);

                    let bounds = var.bounds().unwrap();
                    match bounds.lower {
                        Bound::Bounded(bound) => {
                            e.integer_bounds_lower.push(bound);
                            e.integer_bounds_bounded_lower.push(true);
                        }
                        Bound::Unbounded => e.integer_bounds_bounded_lower.push(false),
                    }
                    match bounds.upper {
                        Bound::Bounded(bound) => {
                            e.integer_bounds_upper.push(bound);
                            e.integer_bounds_bounded_upper.push(true);
                        }
                        Bound::Unbounded => e.integer_bounds_bounded_upper.push(false),
                    }
                }
                Vtype::Real => {
                    e.real.push(i as u32);
                    e.real_names.push(vname);

                    let bounds = var.bounds().unwrap();
                    match bounds.lower {
                        Bound::Bounded(bound) => {
                            e.real_bounds_lower.push(bound);
                            e.real_bounds_bounded_lower.push(true);
                        }
                        Bound::Unbounded => e.real_bounds_bounded_lower.push(false),
                    }
                    match bounds.upper {
                        Bound::Bounded(bound) => {
                            e.real_bounds_upper.push(bound);
                            e.real_bounds_bounded_upper.push(true);
                        }
                        Bound::Unbounded => e.real_bounds_bounded_upper.push(false),
                    }
                }
            }
        }
        e.encode_to_vec()
    }
}
