use crate::{
    core::{environment::SharedEnvironment, Model, Sense},
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, Creatable, DecodeError},
        Decodable, Encodable,
    },
};
use prost::Message;
use std::{ops::Deref, str::FromStr};

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

/// Makes the SerModel conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerModel {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerModel conform with the requirements for it to be an Decodable.
impl BytesDecodable<Model> for SerModel {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<Model, DecodeError> {
        Self::decode(bytes)?.extract()
    }
}

/// Makes the SerModel conform with the requirements for it to be an Encodable.
impl Creatable<Model> for SerModel {
    fn new(value: &Model) -> Self {
        Self::empty(value.name.clone(), value.sense).fill(&value)
    }
}

impl SerModel {
    /// Creates an empty serializable model struct.
    fn empty(name: String, sense: Sense) -> Self {
        Self {
            objective: Vec::new(),
            constraints: Vec::new(),
            environment: Vec::new(),
            sense: sense.to_string(),
            name,
        }
    }

    /// Fills the serializable model based on an instance of Model.
    fn fill(mut self, model: &Model) -> Self {
        self.objective = model.objective.serialize();
        self.constraints = model.constraints.serialize();
        self.environment = model.environment.access().deref().serialize();
        self
    }

    /// Extracts the data from self to an instance of Model with Index VarId and
    /// Bias f64.
    pub fn extract(&self) -> Result<Model, DecodeError> {
        let sense = Sense::from_str(&self.sense).map_err(|e| DecodeError::new(e.to_string()))?;
        let mut model = Model::new(Some(self.name.clone()), Some(sense));
        model.environment = SharedEnvironment::from(self.environment.decode(())?);
        model.objective = self.objective.decode(model.environment.clone())?;
        model.constraints = self.constraints.decode(model.environment.clone())?;
        Ok(model)
    }
}
