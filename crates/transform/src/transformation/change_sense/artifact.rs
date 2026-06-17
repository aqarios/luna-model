//! Artifact types for objective-sense normalization.

use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::Artifact;

pub struct ChangeSensePassArtifact {
    pub(super) did_change: bool,
}

impl ChangeSensePassArtifact {
    pub fn did_change(&self) -> bool {
        self.did_change
    }
}

impl Artifact for ChangeSensePassArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::change-sense"
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
