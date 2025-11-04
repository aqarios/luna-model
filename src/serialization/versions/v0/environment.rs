use crate::{
    core::{environment::ENV_COUNTER, Bound, Environment, LazyBounds, VarId, Variable, Vtype},
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, Creatable},
        utils::force_u32,
    },
};
use hashbrown::HashMap;
use prost::Message;

/// Representation of a bytes encodable/decodable environment.
#[derive(Clone, PartialEq, Message)]
pub struct SerEnvironment {
    // NOTE: Old "id" field has been removed
    // id: u32, // tag = 1 (Removed)
    /// The number of variables registered in the environment.
    #[prost(uint32, tag = "2")]
    varcount: u32,

    /// The indices of the binary variables.
    #[prost(uint32, repeated, tag = "3")]
    binary: Vec<u32>,
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
    /// The names of the inverted binary variables
    #[prost(string, repeated, tag = "22")]
    inverted_binary_names: Vec<String>,
    /// The names of the spin variables
    #[prost(string, repeated, tag = "8")]
    pub spin_names: Vec<String>,
    /// The names of the integer variables
    #[prost(string, repeated, tag = "9")]
    pub integer_names: Vec<String>,
    /// The names of the real variables
    #[prost(string, repeated, tag = "10")]
    pub real_names: Vec<String>,

    /// If the integer at each index has a non-default bound.
    #[prost(bool, repeated, tag = "11")]
    pub integer_bounds_bounded_lower: Vec<bool>,
    /// If the integer at each index has a non-default bound.
    #[prost(bool, repeated, tag = "12")]
    pub integer_bounds_bounded_upper: Vec<bool>,
    /// The integer lower bounds
    #[prost(double, repeated, tag = "13")]
    pub integer_bounds_lower: Vec<f64>,
    /// The integer upper bounds
    #[prost(double, repeated, tag = "14")]
    pub integer_bounds_upper: Vec<f64>,

    /// If the real at each index has a non-default bound.
    #[prost(bool, repeated, tag = "15")]
    pub real_bounds_bounded_lower: Vec<bool>,
    /// If the real at each index has a non-default bound.
    #[prost(bool, repeated, tag = "16")]
    pub real_bounds_bounded_upper: Vec<bool>,
    /// The reals' lower bounds.
    #[prost(double, repeated, tag = "17")]
    pub real_bounds_lower: Vec<f64>,
    /// The reals' upper bounds.
    #[prost(double, repeated, tag = "18")]
    pub real_bounds_upper: Vec<f64>,

    /// The length of the variables vector.
    #[prost(uint64, tag = "19")]
    variables_len: u64,
    /// The indices of the ghost variables.
    #[prost(uint32, repeated, tag = "20")]
    ghosts: Vec<u32>,
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
        let mut out = Self::base(
            environment.varcount(),
            environment.all_variables().len() as u64,
        );

        for (i, var) in environment.all_variables().enumerate() {
            match var.vtype {
                Vtype::__Ghost => {
                    out.ghosts.push(force_u32(i));
                }
                Vtype::Binary => {
                    out.binary.push(force_u32(i));
                    out.binary_names.push(var.name.clone());
                }
                Vtype::InvertedBinary=> {
                    out.inverted_binary.push(force_u32(i));
                    out.inverted_binary_names.push(var.name.clone());
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
    fn base(varcount: u32, variables_len: u64) -> Self {
        Self {
            varcount,
            variables_len,
            binary: Vec::new(),
            inverted_binary: Vec::new(),
            spin: Vec::new(),
            integer: Vec::new(),
            real: Vec::new(),
            binary_names: Vec::new(),
            inverted_binary_names: Vec::new(),
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
            ghosts: Vec::new(),
        }
    }

    /// Extracts the data from self to and instance of Environment with Index VarId.
    pub fn extract(&self) -> Environment {
        // Serialization UPDATE
        let mut env = Environment::new_for(ENV_COUNTER.inc());
        let varcount = VarId(self.varcount);
        let mut variables_lookup = HashMap::new();
        let mut variables = Vec::with_capacity(self.variables_len as usize);
        let mut ghost_vars = Vec::with_capacity(self.ghosts.len());
        let mut inverted_vars = Vec::with_capacity(self.inverted_binary.len());
        let num_vars = if self.variables_len != 0 {
            self.variables_len as usize
        } else {
            self.varcount as usize
        };
        variables.resize(num_vars, Variable::default());

        for i in self.ghosts.iter() {
            variables[*i as usize] = Variable::ghost();
            ghost_vars.push(*i as usize);
        }

        for (i, v) in self.binary.iter().enumerate() {
            let name = self.binary_names[i].clone();
            variables[*v as usize] =
                Variable::new(name.clone(), Some(Vtype::Binary), None, env.id())
                    .expect("binary variable creation failed during deserialization");
            variables_lookup.insert(name, VarId(*v));
        }
        for (i, v) in self.inverted_binary.iter().enumerate() {
            let name = self.inverted_binary_names[i].clone();
            variables[*v as usize] =
                Variable::new(name.clone(), Some(Vtype::InvertedBinary), None, env.id())
                    .expect("inverted binary variable creation failed during deserialization");
            variables_lookup.insert(name, VarId(*v));
            inverted_vars.push(*v as usize);
        }
        for (i, v) in self.spin.iter().enumerate() {
            let name = self.spin_names[i].clone();
            variables[*v as usize] = Variable::new(name.clone(), Some(Vtype::Spin), None, env.id())
                .expect("spin variable creation failed during deserialization");
            variables_lookup.insert(name, VarId(*v));
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
            variables[*v as usize] = Variable::new(
                name.clone(),
                Some(Vtype::Integer),
                Some(LazyBounds::new(lower, upper)),
                env.id(),
            )
            .expect("integer variable creation failed during deserialization");
            variables_lookup.insert(name, VarId(*v));
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
            variables[*v as usize] = Variable::new(
                name.clone(),
                Some(Vtype::Real),
                Some(LazyBounds::new(lower, upper)),
                env.id(),
            )
            .expect("real variable creation failed during deserialization");
            variables_lookup.insert(name, VarId(*v));
        }

        env.set_data(varcount, variables, variables_lookup, ghost_vars, inverted_vars);
        env
    }
}
