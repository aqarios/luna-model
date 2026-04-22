use prost::Message;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::TransformationRecord;

use crate::{encode::BytesDecodable, versions::v0::transformation_record::pass_entry::SerPassEntry};

use super::SerTransformationRecord;

impl BytesDecodable<TransformationRecord> for SerTransformationRecord {
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<TransformationRecord> {
        Self::decode(bytes)?.extract()
    }
}

impl SerTransformationRecord {
    fn extract(&self) -> LunaModelResult<TransformationRecord> {
        Ok(self
            .entries
            .iter()
            .map(|buf| match SerPassEntry::decode(buf.as_slice()) {
                Ok(entry) => entry.extract(),
                Err(e) => Err(e.into()),
            })
            .collect::<LunaModelResult<Vec<_>>>()?
            .into())
    }
}
