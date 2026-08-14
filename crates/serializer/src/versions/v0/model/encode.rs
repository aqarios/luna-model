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
        // Nested sub-encoded fields are tagged with their own version (but not
        // independently compressed - see .encode()) so a decoder can dispatch
        // each field to the right version-specific decoder even if the inner
        // type's serialization format has moved ahead of the outer SerModel
        // format. See Issue(474): <https://github.com/aqarios/luna-model/issues/474>
        self.objective = m.objective.serialize().versionize(m.objective.version());
        self.constraints = m
            .constraints
            .serialize()
            .versionize(m.constraints.version());
        let env = m.environment.read_arc();
        self.environment = env.serialize().versionize(env.version());
        self.name = m.name.clone();
        self.sense = m.sense.to_string();
        self
    }
}
