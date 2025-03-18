use crate::serialization::encodable::BytesDecodable;
use crate::{
    core::{Comparator, Constraint, Constraints, Environment, VarId},
    serialization::{
        encodable::{BytesEncodable, Creatable, DecodeError},
        Decodable, Encodable,
    },
};
use prost::Message;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Message)]
pub struct SerConstraints {
    /// All serialized lhs (expressions) as concatenated bytes.
    #[prost(bytes, repeated, tag = "1")]
    lhsides: Vec<Vec<u8>>,
    /// The rhs for each constraint. This length is equal to the number of constraints.
    #[prost(double, repeated, tag = "2")]
    rhsides: Vec<f64>,
    /// The comparator for each constraint used. Equal to number of constraints.
    #[prost(uint32, repeated, tag = "3")]
    comparators: Vec<u32>,
}

impl BytesEncodable for SerConstraints {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

type RefEnv = Rc<RefCell<Environment<VarId>>>;

impl BytesDecodable<Constraints<VarId, f64>, RefEnv> for SerConstraints {
    fn decode_from_bytes(
        bytes: &[u8],
        payload: RefEnv,
    ) -> Result<Constraints<VarId, f64>, DecodeError> {
        Self::decode(bytes)?.extract(payload)
    }
}

impl Creatable<Constraints<VarId, f64>> for SerConstraints {
    fn new(value: &Constraints<VarId, f64>) -> Self {
        Self::default().fill(value)
    }
}

impl SerConstraints {
    fn default() -> Self {
        Self {
            lhsides: Vec::new(),
            rhsides: Vec::new(),
            comparators: Vec::new(),
        }
    }

    fn fill(mut self, constraints: &Constraints<VarId, f64>) -> Self {
        for c in &constraints.constraints {
            let lhs_bytes = c.lhs.borrow().encode();

            let comparator = match c.comparator {
                Comparator::Leq => 0,
                Comparator::Eq => 1,
                Comparator::Geq => 2,
            };
            self.lhsides.push(lhs_bytes);
            self.rhsides.push(c.rhs);
            self.comparators.push(comparator);
        }

        self
    }

    pub fn extract(&self, env: RefEnv) -> Result<Constraints<VarId, f64>, DecodeError> {
        let mut constraints = Vec::new();

        for ((lhs, rhs), comp) in self
            .lhsides
            .iter()
            .zip(&self.rhsides)
            .zip(&self.comparators)
        {
            let lhs_base = lhs.decode(Rc::clone(&env))?;
            let lhs = Rc::new(RefCell::new(lhs_base));
            let comparator = match comp {
                0 => Comparator::Leq,
                1 => Comparator::Eq,
                2 => Comparator::Geq,
                _ => panic!("undefined comparator '{}'", comp),
            };
            constraints.push(Constraint::new(lhs, *rhs, comparator));
        }

        Ok(Constraints::new_from_vec(constraints))
    }
}
