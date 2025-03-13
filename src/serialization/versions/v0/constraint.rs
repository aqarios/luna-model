use std::{cell::RefCell, io, rc::Rc};

use crate::{
    core::{Comparator, Constraint, Constraints, Environment, VarId},
    serialization::{
        encodable::{BytesEncodable, Creatable},
        Encodable,
    },
};
use prost::{DecodeError, Message};

#[derive(Clone, PartialEq, Message)]
pub struct SerConstraints {
    /// All serialized lhs (expressions) as concatenated bytes.
    #[prost(bytes, repeated, tag = "1")]
    lhsides: Vec<Vec<u8>>,
    // /// The lengths for each of the encoded expressions. The length of this
    // /// array is equal to the number of constraints.
    // #[prost(uint64, repeated, tag = "2")]
    // lhsides_lens: Vec<u64>,
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

impl Creatable<Constraints<VarId, f64>> for SerConstraints {
    fn new(value: &Constraints<VarId, f64>) -> Self {}
}

impl SerConstraints {
    pub fn new(
        constraints: &Constraints<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        Self::default().fill(constraints, use_compression, level)
    }

    fn default() -> Self {
        Self {
            lhsides: Vec::new(),
            // lhsides_lens: Vec::new(),
            rhsides: Vec::new(),
            comparators: Vec::new(),
        }
    }

    fn fill(
        mut self,
        constraints: &Constraints<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        for c in &constraints.constraints {
            let lhs_bytes = c.lhs.borrow().encode(use_compression, level)?;
            let comparator = match c.comparator {
                Comparator::Leq => 0,
                Comparator::Eq => 1,
                Comparator::Geq => 2,
            };
            // self.lhsides_lens.push(lhs_bytes.len() as u64);
            self.lhsides.push(lhs_bytes);
            self.rhsides.push(c.rhs);
            self.comparators.push(comparator);
        }

        Ok(self)
    }

    pub fn extract(
        &self,
        env: Rc<RefCell<Environment<VarId>>>,
    ) -> Result<Constraints<VarId, f64>, DecodeError> {
        let mut constraints = Vec::new();

        for ((lhs, rhs), comp) in self
            .lhsides
            .iter()
            .zip(&self.rhsides)
            .zip(&self.comparators)
        {
            let lhs_base = decode_expression(lhs, Rc::clone(&env))?;
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
