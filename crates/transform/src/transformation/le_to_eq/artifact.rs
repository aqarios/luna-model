use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::Artifact;

#[derive(Default)]
pub struct LeToEqConstraintsArtifact {
    pub(super) slackvars: Vec<String>,
}

impl LeToEqConstraintsArtifact {
    pub fn slackvars(&self) -> &[String] {
        &self.slackvars
    }
}

impl Artifact for LeToEqConstraintsArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::equality-constraints-to-quadratic-penalty"
    }

    fn serialize(&self) -> LunaModelResult<Vec<u8>> {
        self.encode(Some(true), Some(3))
    }

    fn deserialize(bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized,
    {
        bytes.unversionize().decompress()?.decode(())
    }
}
