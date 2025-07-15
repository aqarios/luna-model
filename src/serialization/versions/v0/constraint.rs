use crate::core::environment::SharedEnvironment;
use crate::core::{Constraint, Constraints};
use crate::serialization::encodable::BytesDecodable;
use crate::{
    core::Comparator,
    serialization::{
        encodable::{BytesEncodable, Creatable, DecodeError},
        Decodable, Encodable,
    },
};
use prost::Message;

static PLACEHOLDER_NAME: &str = "<NN>";

/// Representation of encodable constraints based on protocol buffers.
#[derive(Clone, PartialEq, Message)]
pub struct SerConstraints {
    /// Representation of the left-hand-sides of all constraints as a vector of byte
    /// vectors. Each byte vector (Vec<u8>) is an encoded expression.
    #[prost(bytes, repeated, tag = "1")]
    lhsides: Vec<Vec<u8>>,
    /// Representation of the right-hand-sides of all constraints as a vector of double
    /// values (f64).
    #[prost(double, repeated, tag = "2")]
    rhsides: Vec<f64>,
    /// Representation of the comparator used for all constraints. The comparator is
    /// encoded using the minimally possible data type available in this protobuf
    /// implementation which is a u32.
    #[prost(uint32, repeated, tag = "3")]
    comparators: Vec<u32>,
    /// Representation of the constraint names used for all constraints.
    #[prost(string, repeated, tag = "4")]
    names: Vec<String>,
}

/// Makes the SerConstraints conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerConstraints {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerConstraints conform with the requirements for it to be a Decodable.
/// The result is a Constraints<VarId, f64> instance.
impl BytesDecodable<Constraints, SharedEnvironment> for SerConstraints {
    fn decode_from_bytes(
        bytes: &[u8],
        payload: SharedEnvironment,
    ) -> Result<Constraints, DecodeError> {
        Self::decode(bytes)?.extract(payload)
    }
}

/// Makes the SerConstraints conform with the requirements for it to be an Encodable.
impl Creatable<Constraints> for SerConstraints {
    fn new(value: &Constraints) -> Self {
        Self::default().fill(value)
    }
}

impl SerConstraints {
    /// Creates an empty serializable constraints struct.
    fn default() -> Self {
        Self {
            lhsides: Vec::new(),
            rhsides: Vec::new(),
            comparators: Vec::new(),
            names: Vec::new(),
        }
    }

    /// Fills the serializable constraints based on an instance of constraints.
    fn fill(mut self, constraints: &Constraints) -> Self {
        for c in &constraints.constraints {
            let lhs_bytes = c.lhs.serialize();

            let comparator = match c.comparator {
                Comparator::Le => 0,
                Comparator::Eq => 1,
                Comparator::Ge => 2,
            };
            self.lhsides.push(lhs_bytes);
            self.rhsides.push(c.rhs);
            self.comparators.push(comparator);
            self.names
                .push(c.name.clone().unwrap_or(PLACEHOLDER_NAME.to_string()));
        }

        self
    }

    /// Extracts the data from self to an instance of Constraints with Index VarId and
    /// Bias f64.
    pub fn extract(&self, env: SharedEnvironment) -> Result<Constraints, DecodeError> {
        let mut constraints = Vec::new();

        for (((lhs, rhs), comp), name) in self
            .lhsides
            .iter()
            .zip(&self.rhsides)
            .zip(&self.comparators)
            .zip(&self.names)
        {
            let lhs = lhs.decode(env.clone())?;
            let comparator = match comp {
                0 => Comparator::Le,
                1 => Comparator::Eq,
                2 => Comparator::Ge,
                _ => panic!("undefined comparator '{}'", comp),
            };
            let name = if name == PLACEHOLDER_NAME {
                None
            } else {
                Some(name.clone())
            };
            constraints.push(Constraint::new(lhs, *rhs, comparator, name)?);
        }

        Ok(Constraints::new_from_vec(constraints))
    }
}
