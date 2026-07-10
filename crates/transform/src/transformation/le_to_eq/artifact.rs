//! Artifact types for `<=` to equality conversion.

use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::{Artifact, TranspileErrorKind, TranspileKindResult};

#[derive(Default)]
pub struct LeToEqConstraintsArtifact {
    pub(super) slackvars: Vec<String>,
}

impl LeToEqConstraintsArtifact {
    /// Returns the names of all generated slack variables.
    pub fn slackvars(&self) -> &[String] {
        &self.slackvars
    }
}

impl Artifact for LeToEqConstraintsArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::equality-constraints-to-quadratic-penalty"
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
