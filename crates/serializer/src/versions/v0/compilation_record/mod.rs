mod pass_entry;
mod artifact;

mod decode;
mod encode;

use lunamodel_transpiler::CompilationRecord;
use prost::Message;

use crate::encode::Creatable;

#[derive(Clone, PartialEq, Message)]
pub struct SerCompilationRecord {
    #[prost(bytes, repeated, tag = "1")]
    entries: Vec<Vec<u8>>,
}

impl Creatable<CompilationRecord> for SerCompilationRecord {
    fn new(value: &CompilationRecord) -> Self {
        Self::default().fill(&value)
    }
}

