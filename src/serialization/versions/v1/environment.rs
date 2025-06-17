use crate::core::environment::ENV_COUNTER;
use crate::{
    core::{Bound, Environment, LazyBounds, VarId, Variable, Vtype},
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, Creatable},
        utils::{force_u32, force_u8},
    },
};
use prost::Message;

/// Representation of a bytes encodable/decodable environment.
#[derive(Clone, PartialEq, Message)]
pub struct SerEnvironment {
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

/// Makes the SerEnvironment conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerEnvironment {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerEnvironment conform with the requirements for it to be a Decodable.
impl BytesDecodable<Environment> for SerEnvironment {
    fn decode_from_bytes(
        bytes: &[u8],
        _payload: (),
    ) -> Result<Environment, crate::serialization::encodable::DecodeError> {
        Ok(Self::decode(bytes)?.extract())
    }
}

/// Makes the SerEnvironment conform with the requirements for it to be an Encodable.
impl Creatable<Environment> for SerEnvironment {
    /// Creates a new instance of a serializabl environment and fills it based on an
    /// instance of Environment.
    fn new(environment: &Environment) -> Self {
        let mut out = Self::base(environment.varcount.0);

        for (i, var) in environment.variables.iter().enumerate() {
            match var.vtype {
                Vtype::Binary => {
                    out.binary.push(force_u32(i));
                    out.binary_names.push(var.name.clone());
                }
                Vtype::Spin => {
                    out.spin.push(force_u32(i));
                    out.spin_names.push(var.name.clone());
                }
                Vtype::Integer => {
                    out.integer.push(force_u32(i));
                    out.integer_names.push(var.name.clone());

                    if var.bounds.lower.is_bounded() {
                        out.integer_bounds_bounded_lower.push(true);
                        out.integer_bounds_lower.push(var.bounds.lower.unwrap());
                    } else {
                        out.integer_bounds_bounded_lower.push(false);
                    }
                    if var.bounds.upper.is_bounded() {
                        out.integer_bounds_bounded_upper.push(true);
                        out.integer_bounds_upper.push(var.bounds.upper.unwrap());
                    } else {
                        out.integer_bounds_bounded_upper.push(false);
                    }
                }
                Vtype::Real => {
                    out.real.push(force_u32(i));
                    out.real_names.push(var.name.clone());
                    if var.bounds.lower.is_bounded() {
                        out.real_bounds_bounded_lower.push(true);
                        out.real_bounds_lower.push(var.bounds.lower.unwrap());
                    } else {
                        out.real_bounds_bounded_lower.push(false);
                    }
                    if var.bounds.upper.is_bounded() {
                        out.real_bounds_bounded_upper.push(true);
                        out.real_bounds_upper.push(var.bounds.upper.unwrap());
                    } else {
                        out.real_bounds_bounded_upper.push(false);
                    }
                }
            }
        }

        out
    }
}

impl SerEnvironment {
    /// Creates an empty serializable environment.
    fn base(varcount: u32) -> Self {
        Self {
            varcount,
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
        }
    }

    /// Extracts the data from self to and instance of Environment with Index VarId.
    pub fn extract(&self) -> Environment {
        // Serialization UPDATE
        let mut env = Environment::new_for(ENV_COUNTER.inc());
        env.varcount = VarId(self.varcount);
        env.variables = Vec::with_capacity(self.varcount as usize);
        env.variables
            .resize(self.varcount as usize, Variable::default());

        for (i, v) in self.binary.iter().enumerate() {
            let name = self.binary_names[i].clone();
            env.variables[*v as usize] =
                Variable::new(name.clone(), Some(Vtype::Binary), None, env.id)
                    .expect("binary variable creation failed during deserialization");
            env.variables_lookup.insert(name, VarId(*v));
        }
        for (i, v) in self.spin.iter().enumerate() {
            let name = self.spin_names[i].clone();
            env.variables[*v as usize] =
                Variable::new(name.clone(), Some(Vtype::Spin), None, env.id)
                    .expect("spin variable creation failed during deserialization");
            env.variables_lookup.insert(name, VarId(*v));
        }
        let mut int_pos_lower = 0;
        let mut int_pos_upper = 0;
        for (i, v) in self.integer.iter().enumerate() {
            let lower = self.integer_bounds_bounded_lower[i]
                .then(|| {
                    let out = self.integer_bounds_lower[int_pos_lower];
                    int_pos_lower += 1;
                    Bound::Some(out)
                })
                .or(Some(Bound::Unbounded()));
            let upper = self.integer_bounds_bounded_upper[i]
                .then(|| {
                    let out = self.integer_bounds_upper[int_pos_upper];
                    int_pos_upper += 1;
                    Bound::Some(out)
                })
                .or(Some(Bound::Unbounded()));
            let name = self.integer_names[i].clone();
            env.variables[*v as usize] = Variable::new(
                name.clone(),
                Some(Vtype::Integer),
                Some(LazyBounds::new(lower, upper)),
                env.id,
            )
            .expect("integer variable creation failed during deserialization");
            env.variables_lookup.insert(name, VarId(*v));
        }

        let mut real_pos_lower = 0;
        let mut real_pos_upper = 0;
        for (i, v) in self.real.iter().enumerate() {
            let lower = self.real_bounds_bounded_lower[i]
                .then(|| {
                    let out = self.real_bounds_lower[real_pos_lower];
                    real_pos_lower += 1;
                    Bound::Some(out)
                })
                .or(Some(Bound::Unbounded()));
            let upper = self.real_bounds_bounded_upper[i]
                .then(|| {
                    let out = self.real_bounds_upper[real_pos_upper];
                    real_pos_upper += 1;
                    Bound::Some(out)
                })
                .or(Some(Bound::Unbounded()));
            let name = self.real_names[i].clone();
            env.variables[*v as usize] = Variable::new(
                name.clone(),
                Some(Vtype::Real),
                Some(LazyBounds::new(lower, upper)),
                env.id,
            )
            .expect("real variable creation failed during deserialization");
            env.variables_lookup.insert(name, VarId(*v));
        }

        env
    }
}
