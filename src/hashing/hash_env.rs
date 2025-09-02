use prost::Message;

use crate::core::{SharedEnvironment, Vtype};

use super::utils::force_u32;

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
    pub fn build(env: &SharedEnvironment) -> Vec<u8> {
        let mut serenv = HashEnv {
            // the id was 0 (zero) for all environments in a past version. So we set it to exactly
            // this value.
            id: 0,
            varcount: env.varcount().0,
            binary: Vec::new(),
            spin: Vec::new(),
            integer: Vec::new(),
            real: Vec::new(),
            binary_names: Vec::new(),
            spin_names: Vec::new(),
            integer_names: Vec::new(),
            real_names: Vec::new(),
            integer_bounds_bounded_lower: Vec::new(),
            integer_bounds_bounded_upper: Vec::new(),
            integer_bounds_lower: Vec::new(),
            integer_bounds_upper: Vec::new(),
            real_bounds_bounded_lower: Vec::new(),
            real_bounds_bounded_upper: Vec::new(),
            real_bounds_lower: Vec::new(),
            real_bounds_upper: Vec::new(),
        };

        for (i, var) in env.access().all_variables().enumerate() {
            match var.vtype {
                Vtype::__Ghost => (),
                Vtype::Binary => {
                    serenv.binary.push(force_u32(i));
                    serenv.binary_names.push(var.name.clone());
                }
                Vtype::Spin => {
                    serenv.spin.push(force_u32(i));
                    serenv.spin_names.push(var.name.clone());
                }
                Vtype::Integer => {
                    serenv.integer.push(force_u32(i));
                    serenv.integer_names.push(var.name.clone());

                    if var.bounds.lower.is_bounded() {
                        serenv.integer_bounds_bounded_lower.push(true);
                        serenv.integer_bounds_lower.push(var.bounds.lower.unwrap());
                    } else {
                        serenv.integer_bounds_bounded_lower.push(false);
                    }
                    if var.bounds.upper.is_bounded() {
                        serenv.integer_bounds_bounded_upper.push(true);
                        serenv.integer_bounds_upper.push(var.bounds.upper.unwrap());
                    } else {
                        serenv.integer_bounds_bounded_upper.push(false);
                    }
                }
                Vtype::Real => {
                    serenv.real.push(force_u32(i));
                    serenv.real_names.push(var.name.clone());
                    if var.bounds.lower.is_bounded() {
                        serenv.real_bounds_bounded_lower.push(true);
                        serenv.real_bounds_lower.push(var.bounds.lower.unwrap());
                    } else {
                        serenv.real_bounds_bounded_lower.push(false);
                    }
                    if var.bounds.upper.is_bounded() {
                        serenv.real_bounds_bounded_upper.push(true);
                        serenv.real_bounds_upper.push(var.bounds.upper.unwrap());
                    } else {
                        serenv.real_bounds_bounded_upper.push(false);
                    }
                }
            }
        }

        serenv.encode_to_vec()
    }
}
