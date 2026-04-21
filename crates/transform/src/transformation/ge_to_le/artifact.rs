use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::Artifact;

pub struct GeToLeConstraintsArtifact;

impl Artifact for GeToLeConstraintsArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::ge-to-le-constraints"
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
