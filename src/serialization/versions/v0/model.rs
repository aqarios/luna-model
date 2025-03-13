use crate::{
    core::{Model, VarId},
    serialization::{
        encodable::{BytesDecodable, BytesEncodable, Creatable, DecodeError},
        Decodable, Encodable,
    },
};
use prost::Message;
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Clone, PartialEq, Message)]
pub struct SerModel {
    /// The model's objective serialized and maybe compressed
    #[prost(bytes, tag = "1")]
    objective: Vec<u8>,
    /// The model's constraints serialized and maybe compressed
    #[prost(bytes, tag = "2")]
    constraints: Vec<u8>,
    /// The model's environment serialized and maybe compressed
    #[prost(bytes, tag = "3")]
    environment: Vec<u8>,
    /// The model's name
    #[prost(string, tag = "4")]
    name: String,
}

impl BytesEncodable for SerModel {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<Model<VarId, f64>> for SerModel {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<Model<VarId, f64>, DecodeError> {
        Self::decode(bytes)?.extract()
    }
}

impl Creatable<Model<VarId, f64>> for SerModel {
    fn new(value: &Model<VarId, f64>) -> Self {
        Self::empty(value.name.clone()).fill(&value)
    }
}

impl SerModel {
    fn empty(name: String) -> Self {
        Self {
            objective: Vec::new(),
            constraints: Vec::new(),
            environment: Vec::new(),
            name,
        }
    }

    fn fill(mut self, model: &Model<VarId, f64>) -> Self {
        self.objective = model.objective.borrow().deref().encode();
        self.constraints = model.constraints.borrow().deref().encode();
        self.environment = model.environment.borrow().deref().encode();
        self
    }

    pub fn extract(&self) -> Result<Model<VarId, f64>, DecodeError> {
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
