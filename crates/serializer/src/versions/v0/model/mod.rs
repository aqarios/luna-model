mod decode;
mod encode;

use lunamodel_core::Model;
use prost::Message;

use crate::encode::Creatable;

/// Representation of encodable model based on protocol buffers.
#[derive(Clone, PartialEq, Message)]
pub struct SerModel {
    /// Representation of the objective as a byte vector, i.e. an encoded Expression.
    #[prost(bytes, tag = "1")]
    objective: Vec<u8>,
    /// Representation of the constraints as a byte vector, i.e. an encoded Constraints.
    #[prost(bytes, tag = "2")]
    constraints: Vec<u8>,
    /// Representation of the environment as a byte vector, i.e., an encoded Environment.
    #[prost(bytes, tag = "3")]
    environment: Vec<u8>,
    /// The name of the model.
    #[prost(string, tag = "4")]
    name: String,
    /// The sense of the model.
    #[prost(string, tag = "5")]
    sense: String,
}

impl Creatable<Model> for SerModel {
    fn new(model: &Model) -> Self {
        Self::default().fill(model)
    }
}
