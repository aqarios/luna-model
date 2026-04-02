use lunamodel_transpiler::CompilationRecord;
use prost::Message;

use crate::{encode::BytesEncodable, versions::v0::compilation_record::pass_entry::SerPassEntry};

use super::SerCompilationRecord;

impl BytesEncodable for SerCompilationRecord {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerCompilationRecord {
    pub fn fill(mut self, value: &CompilationRecord) -> Self {
        for entry in value.entries() {
            self.entries
                .push(SerPassEntry::from(entry).encode_to_bytes());
        }
        self
    }
}
