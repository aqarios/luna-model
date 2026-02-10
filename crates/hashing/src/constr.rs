use lunamodel_core::ConstraintCollection;
use lunamodel_types::Comparator;
use prost::Message;

use crate::expr::HashExpr;

/// Representation of encodable constraints based on protocol buffers.
#[derive(Clone, PartialEq, Message)]
pub struct HashConstr {
    /// Representation of the left-hand-sides of all constraints as a vector of byte
    /// vectors. Each byte vector (`Vec<u8>`) is an encoded expression.
    #[prost(bytes, repeated, tag = "1")]
    lhsides: Vec<Vec<u8>>,
    /// Representation of the right-hand-sides of all constraints as a vector of double
    /// values (`f64`).
    #[prost(double, repeated, tag = "2")]
    rhsides: Vec<f64>,
    /// Representation of the comparator used for all constraints. The comparator is
    /// encoded using the minimally possible data type available in this protobuf
    /// implementation which is a `u32`.
    #[prost(uint32, repeated, tag = "3")]
    comparators: Vec<u32>,
    /// Representation of the constraint names used for all constraints.
    #[prost(string, repeated, tag = "4")]
    names: Vec<String>,
}

impl HashConstr {
    pub fn build(constr: &ConstraintCollection) -> Vec<u8> {
        let mut o = HashConstr {
            lhsides: Vec::new(),
            rhsides: Vec::new(),
            comparators: Vec::new(),
            names: Vec::new(),
        };

        for (_, c) in constr.iter() {
            let lhs = HashExpr::build(&c.lhs);
            let cmp = match c.comparator {
                Comparator::Le => 0,
                Comparator::Eq => 1,
                Comparator::Ge => 2,
            };
            o.lhsides.push(lhs);
            o.rhsides.push(c.rhs);
            o.comparators.push(cmp);
            o.names.push(c.name().to_string());
        }
        o.encode_to_vec()
    }
}
