use lunamodel_transpiler::TransformationRecord;
use prost::Message;

use crate::{encode::BytesEncodable, versions::v0::transformation_record::pass_entry::SerPassEntry};

use super::SerTransformationRecord;

impl BytesEncodable for SerTransformationRecord {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerTransformationRecord {
    pub fn fill(mut self, value: &TransformationRecord) -> Self {
        for entry in value.entries() {
            self.entries
                .push(SerPassEntry::from(entry).encode_to_bytes());
        }
        self
    }
}
