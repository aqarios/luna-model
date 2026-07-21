//! The plugin-side mirror of the transpiler's `Artifact` trait.

/// Encodes the inverse of a transformation. Serialized across the C ABI and stored in the host's
/// transformation record so a solution can be mapped back later.
///
/// A 1:1 mirror of `lunamodel_transpiler::Artifact` (minus the blanket helpers), implemented by
/// plugin authors exactly as they would for a native pass.
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

    /// Serializes this artifact to bytes.
    fn serialize(&self) -> crate::Result<Vec<u8>>;

    /// Reconstructs this artifact from bytes.
    fn deserialize(bytes: &[u8]) -> crate::Result<Self>
    where
        Self: Sized;
}
