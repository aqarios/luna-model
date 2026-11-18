//! The plugin-side mirror of the transpiler's `Artifact` trait.

use prost::Message;

/// Encodes the inverse of a transformation. Serialized across the C ABI and stored in the host's
/// transformation record so a solution can be mapped back later.
///
/// A 1:1 mirror of `lunamodel_transpiler::Artifact` (minus the blanket helpers), implemented by
/// plugin authors exactly as they would for a native pass.
pub trait Artifact: Message + Default + Send + Sync + 'static {
    /// Unique type identifier for this artifact type.
    fn static_type_tag() -> &'static str;

    /// Unique type identifier for this artifact value.
    fn type_tag(&self) -> &'static str {
        Self::static_type_tag()
    }
}
