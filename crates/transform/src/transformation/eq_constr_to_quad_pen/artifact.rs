//! Artifact types for equality-to-quadratic-penalty conversion.

use lunamodel_transpiler::{Artifact, TranspileKindResult};

pub struct EqualityConstraintsToQuadraticPenaltyArtifact;

impl Artifact for EqualityConstraintsToQuadraticPenaltyArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::equality-constraints-to-quadratic-penalty"
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
