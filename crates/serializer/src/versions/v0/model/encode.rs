//! Version 0 encoding for models.

use lunamodel_core::Model;
use prost::Message;

use crate::encode::{BytesEncodable, Encodable};
use crate::versionize::Versionizable;

use super::SerModel;

impl BytesEncodable for SerModel {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerModel {
    pub fn fill(mut self, m: &Model) -> Self {
        self.objective = m
            .objective
            .serialize()
            .versionize_nested(m.objective.version());
        self.constraints = m
            .constraints
            .serialize()
            .versionize_nested(m.constraints.version());
        let env = m.environment.read_arc();
        self.environment = env.serialize().versionize_nested(env.version());
        self.name = m.name.clone();
        self.sense = m.sense.to_string();
        self
    }
}
