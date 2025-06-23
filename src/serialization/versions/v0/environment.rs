use crate::{
    core::{environment::ENV_COUNTER, Bound, Environment, LazyBounds, VarId, Variable, Vtype},
    serialization::encodable::BytesDecodable,
};
use prost::Message;

/// Representation of a bytes encodable/decodable environment.
#[derive(Clone, PartialEq, Message)]
pub struct SerEnvironment {
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

/// Makes the SerEnvironment conform with the requirements for it to be a Decodable.
impl BytesDecodable<Environment> for SerEnvironment {
    fn decode_from_bytes(
        bytes: &[u8],
        _payload: (),
    ) -> Result<Environment, crate::serialization::encodable::DecodeError> {
        Ok(Self::decode(bytes)?.extract())
    }
}

impl SerEnvironment {
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
