use crate::{
    core::{ConcreteModel, Model},
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, Creatable, DecodeError},
        Decodable, Encodable,
    },
};
use prost::Message;
use std::{cell::RefCell, ops::Deref, rc::Rc};

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
}

/// Makes the SerModel conform with the requirements for it to be an Encodable.
impl BytesEncodable for SerModel {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

/// Makes the SerModel conform with the requirements for it to be an Decodable.
impl BytesDecodable<ConcreteModel> for SerModel {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<ConcreteModel, DecodeError> {
        Self::decode(bytes)?.extract()
    }
}

/// Makes the SerModel conform with the requirements for it to be an Encodable.
impl Creatable<ConcreteModel> for SerModel {
    fn new(value: &ConcreteModel) -> Self {
        Self::empty(value.name.clone()).fill(&value)
    }
}

impl SerModel {
    /// Creates an empty serializable model struct.
    fn empty(name: String) -> Self {
        Self {
            objective: Vec::new(),
            constraints: Vec::new(),
            environment: Vec::new(),
            name,
        }
    }

    /// Fills the serializable model based on an instance of Model.
    fn fill(mut self, model: &ConcreteModel) -> Self {
        self.objective = model.objective.borrow().deref().encode();
        self.constraints = model.constraints.borrow().deref().encode();
        self.environment = model.environment.borrow().deref().encode();
        self
    }

    /// Extracts the data from self to an instance of Model with Index VarId and
    /// Bias f64.
    pub fn extract(&self) -> Result<ConcreteModel, DecodeError> {
        let mut model = Model::new(Some(self.name.clone()));
        model.environment = Rc::new(RefCell::new(self.environment.decode(())?));
        model.objective = Rc::new(RefCell::new(
            self.objective.decode(Rc::clone(&model.environment))?,
        ));
        model.constraints = Rc::new(RefCell::new(
            self.constraints.decode(Rc::clone(&model.environment))?,
        ));
        Ok(model)
    }
}
