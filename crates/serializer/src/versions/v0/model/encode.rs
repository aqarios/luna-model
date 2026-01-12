use lunamodel_core::Model;
use prost::Message;

use crate::encode::{BytesEncodable, Encodable};

use super::SerModel;

impl BytesEncodable for SerModel {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerModel {
    pub fn fill(mut self, m: &Model) -> Self {
        self.objective = m.objective.serialize();
        self.constraints = m.constraints.serialize();
        self.environment = m.environment.read_arc().serialize();
        self.name = m.name.clone();
        self.sense = m.sense.to_string();
        self
    }
}
