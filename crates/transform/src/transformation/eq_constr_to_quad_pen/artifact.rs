//! Artifact types for equality-to-quadratic-penalty conversion.

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::Artifact;

pub struct EqualityConstraintsToQuadraticPenaltyArtifact;

impl Artifact for EqualityConstraintsToQuadraticPenaltyArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::equality-constraints-to-quadratic-penalty"
    }

    fn serialize(&self) -> LunaModelResult<Vec<u8>> {
        Ok(Vec::new())
    }

    fn deserialize(_: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
