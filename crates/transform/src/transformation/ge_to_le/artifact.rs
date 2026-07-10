//! Artifact types for `>=` to `<=` constraint normalization.

use lunamodel_transpiler::{Artifact, TranspileKindResult};

pub struct GeToLeConstraintsArtifact;

impl Artifact for GeToLeConstraintsArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::ge-to-le-constraints"
    }

    fn serialize(&self) -> TranspileKindResult<Vec<u8>> {
        Ok(Vec::new())
    }

    fn deserialize(_: &[u8]) -> TranspileKindResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
