//! Artifact types for inverted-binary reduction.

use lunamodel_transpiler::{Artifact, TranspileKindResult};

pub struct ReduceInvertedBinaryPassArtifact {}

impl Artifact for ReduceInvertedBinaryPassArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::reduce-inverted-binary"
    }

    fn serialize(&self) -> TranspileKindResult<Vec<u8>> {
        Ok(Vec::new())
    }

    fn deserialize(_bytes: &[u8]) -> TranspileKindResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
