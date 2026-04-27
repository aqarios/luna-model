mod artifact;
mod pass_entry;

mod decode;
mod encode;

use lunamodel_transpiler::TransformationRecord;
use prost::Message;

use crate::encode::Creatable;

#[derive(Clone, PartialEq, Message)]
pub struct SerTransformationRecord {
    #[prost(bytes, repeated, tag = "1")]
    entries: Vec<Vec<u8>>,
}

impl Creatable<TransformationRecord> for SerTransformationRecord {
    fn new(value: &TransformationRecord) -> Self {
        Self::default().fill(value)
    }
}
