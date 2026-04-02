use prost::Message;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::CompilationRecord;

use crate::{encode::BytesDecodable, versions::v0::compilation_record::pass_entry::SerPassEntry};

use super::SerCompilationRecord;

impl BytesDecodable<CompilationRecord> for SerCompilationRecord {
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<CompilationRecord> {
        Self::decode(bytes)?.extract()
    }
}

impl SerCompilationRecord {
    fn extract(&self) -> LunaModelResult<CompilationRecord> {
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
