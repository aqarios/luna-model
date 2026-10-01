//! Version 1 serializer for constraint collections.
mod decode;
mod encode;

use lunamodel_core::prelude::ConstraintCollection;
use prost::Message;

use crate::encode::Creatable;

#[derive(Clone, PartialEq, Message)]
pub struct SerConstraint {
    /// Representation of the left-hand-side of a constraint as a byte vector (`Vec<u8>`),
    /// an encoded expression.
    #[prost(bytes, tag = "1")]
    pub(super) lhs: Vec<u8>,
    /// Representation of the right-hand-side of a constraint.
    #[prost(double, tag = "2")]
    pub(super) rhs: f64,
    /// Representation of the comparator used by a constraints.
    /// The comparator is encoded using the minimally possible data type available
    /// in this protobuf implementation which is a u8.
    #[prost(uint32, tag = "3")]
    pub(super) cmp: u32,
    /// The name of the constraint
    #[prost(string, tag = "4")]
    pub(super) name: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct SerConstraintCollection {
    #[prost(message, repeated, tag = "1")]
    elements: Vec<SerConstraint>,
}

impl Creatable<ConstraintCollection> for SerConstraintCollection {
    fn new(value: &ConstraintCollection) -> Self {
        Self::default().fill(value)
    }
}
