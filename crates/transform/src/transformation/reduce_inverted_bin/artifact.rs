use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::Artifact;

pub struct ReduceInvertedBinaryPassArtifact {}

impl Artifact for ReduceInvertedBinaryPassArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::reduce-inverted-binary"
    }

    fn serialize(&self) -> LunaModelResult<Vec<u8>> {
        Ok(Vec::new())
    }

    fn deserialize(_bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
