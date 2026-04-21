use lunamodel_error::LunaModelResult;

use crate::error::TransformationError;

/// An artifact encodes the inverse transformation.
///
/// This is serialized alongside the transformed model to enable backwards execution.
pub trait Artifact: Send + Sync + 'static {
    /// Unique type identifier for this artifact value.
    fn type_tag(&self) -> &'static str
    where
        Self: Sized,
    {
        Self::static_type_tag()
    }
    /// Unique type identifier for this artifact type.
    fn static_type_tag() -> &'static str
    where
        Self: Sized;

    /// Serialize this artifact
    fn serialize(&self) -> LunaModelResult<Vec<u8>>;

    /// Deserialize this artifact type
    fn deserialize(bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized;
}

/// Type-erased artifact for storage in TransformationRecord
#[derive(Debug, Clone)]
pub struct ErasedArtifact {
    type_tag: String,
    data: Vec<u8>,
}

impl ErasedArtifact {
    pub fn create(type_tag: String, data: Vec<u8>) -> Self {
        Self { type_tag, data }
    }

    pub fn new<A: Artifact>(artifact: &A) -> LunaModelResult<Self> {
        Ok(Self {
            type_tag: artifact.type_tag().to_string(),
            data: artifact.serialize()?,
        })
    }

    pub fn restore<A: Artifact>(&self) -> LunaModelResult<A> {
        if self.type_tag != A::static_type_tag() {
            return Err(TransformationError::ArtifactTypeMismatch {
                expected: A::static_type_tag().to_string(),
                found: self.type_tag.clone(),
            }
            .into());
        }
        A::deserialize(&self.data)
    }

    pub fn type_tag(&self) -> &str {
        &self.type_tag
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
