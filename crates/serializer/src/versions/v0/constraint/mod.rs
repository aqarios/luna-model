mod decode;
mod encode;

use lunamodel_core::prelude::ConstraintCollection;
use prost::Message;

use crate::encode::Creatable;

#[derive(Clone, PartialEq, Message)]
pub struct SerConstraintCollection {
    /// Representation of the left-hand-sides of all constraints as a vector of byte
    /// vectors. Each byte vector (`Vec<u8>`) is an encoded expression.
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

impl Creatable<ConstraintCollection> for SerConstraintCollection {
    fn new(value: &ConstraintCollection) -> Self {
        Self::default().fill(value)
    }
}
