use std::{cell::RefCell, io, rc::Rc};

use crate::core::{Comparator, Constraint, Constraints, Environment, VarId};
use prost::{DecodeError, Message};

use super::{
    compression::{compress, decompress},
    decode_expression, encode_expression,
};

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

impl SerConstraints {
    fn default() -> Self {
        Self {
            lhsides: Vec::new(),
            // lhsides_lens: Vec::new(),
            rhsides: Vec::new(),
            comparators: Vec::new(),
        }
    }

    fn new(
        constraints: &Constraints<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        Self::default().fill(constraints, use_compression, level)
    }

    fn fill(
        mut self,
        constraints: &Constraints<VarId, f64>,
        use_compression: bool,
        level: Option<i32>,
    ) -> Result<Self, io::Error> {
        for c in &constraints.constraints {
            let lhs_bytes = encode_expression(&c.lhs.borrow(), use_compression, level)?;
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

    fn extract(
        &self,
        env: Rc<RefCell<Environment<VarId>>>,
    ) -> Result<Constraints<VarId, f64>, DecodeError> {
        let mut constraints = Vec::new();

        // let mut start: usize = 0;
        for ((lhs, rhs), comp) in self
            .lhsides
            .iter()
            .zip(&self.rhsides)
            .zip(&self.comparators)
        {
            // let end = start + (*len as usize);
            // let lhs_bytes = &self.lhsides[start..end];
            // println!("lhs_bytes.len() = {:?} | {:?}", lhs_bytes.len(), len);
            // let lhs_base = decode_expression(lhs_bytes, Rc::clone(&env))?;
            let lhs_base = decode_expression(lhs, Rc::clone(&env))
                .expect("well there is something wrong when doing decode here.");
            let lhs = Rc::new(RefCell::new(lhs_base));

            let comparator = match comp {
                0 => Comparator::Leq,
                1 => Comparator::Eq,
                2 => Comparator::Geq,
                _ => panic!("undefined comparator '{}'", comp),
            };

            constraints.push(Constraint::new(lhs, *rhs, comparator));

            // start += 1;
        }

        Ok(Constraints::new_from_vec(constraints))
    }
}

pub fn encode_constraints(
    constraints: &Constraints<VarId, f64>,
    use_compression: bool,
    level: Option<i32>,
) -> Result<Vec<u8>, io::Error> {
    compress(
        SerConstraints::new(constraints, use_compression, level)?.encode_to_vec(),
        use_compression,
        level,
    )
}

pub fn decode_constraints(
    data: &[u8],
    env: Rc<RefCell<Environment<VarId>>>,
) -> Result<Constraints<VarId, f64>, DecodeError> {
    Ok(SerConstraints::decode(decompress(data)?.as_slice())?.extract(env)?)
}
