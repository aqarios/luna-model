//! Artifact types for objective-sense normalization.

use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::{Artifact, TranspileErrorKind, TranspileKindResult};

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
