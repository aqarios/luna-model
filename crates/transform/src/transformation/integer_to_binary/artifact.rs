//! Artifact types for integer-to-binary encoding.

use std::collections::HashMap;

use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::{Artifact, TranspileErrorKind, TranspileKindResult};

#[derive(Default)]
pub struct IntegerToBinaryArtifact {
    pub(super) varmap: HashMap<String, HashMap<String, usize>>,
    pub(super) offsetmap: HashMap<String, i64>,
}
impl IntegerToBinaryArtifact {
    pub fn varmap(&self) -> &HashMap<String, HashMap<String, usize>> {
        &self.varmap
    }

    pub fn offsetmap(&self) -> &HashMap<String, i64> {
        &self.offsetmap
    }
}

impl Artifact for IntegerToBinaryArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::integer-to-binary"
    }

    fn serialize(&self) -> TranspileKindResult<Vec<u8>> {
        Ok(self.encode(Some(true), Some(3))?)
    }

    fn deserialize(bytes: &[u8]) -> TranspileKindResult<Self>
    where
        Self: Sized,
    {
        Ok(bytes
            .unversionize()
            .decompress()
            .map_err(TranspileErrorKind::external)?
            .decode(())?)
    }
}
