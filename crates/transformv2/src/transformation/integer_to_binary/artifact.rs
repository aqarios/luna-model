use std::collections::HashMap;

use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::Artifact;

#[derive(Default)]
pub struct IntegerToBinaryArtifact {
    pub(super) varmap: HashMap<String, HashMap<String, usize>>,
    pub(super) offsetmap: HashMap<String, usize>,
}

impl Artifact for IntegerToBinaryArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::integer-to-binary"
    }

    fn serialize(&self) -> lunamodel_error::LunaModelResult<Vec<u8>> {
        self.encode(Some(true), Some(3))
    }

    fn deserialize(bytes: &[u8]) -> lunamodel_error::LunaModelResult<Self>
    where
        Self: Sized,
    {
        bytes.unversionize().decompress()?.decode(())
    }
}
