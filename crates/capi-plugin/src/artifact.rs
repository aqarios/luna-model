//! Erased artifact for plugin transformation passes.

use lunamodel_transpiler::{Artifact, TranspileErrorKind, TranspileKindResult};

/// The invers-transformation artifact produced by a plugin `forward`.
///
/// Carries the plugin's stable pass id (used to route reversal to the matching handler) plus the
/// opaque payload bytes the plugin emitted. Framed with a trivial length prefix so it slots into
/// the serializable `TransformationRecord` like any built-in artifact.
pub struct CPluginArtifact {
    pub(crate) plugin_pass_id: String,
    pub(crate) payload: Vec<u8>,
}

impl Artifact for CPluginArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::CPluginArtifact"
    }

    fn serialize(&self) -> TranspileKindResult<Vec<u8>> {
        let id = self.plugin_pass_id.as_bytes();
        let mut out = Vec::with_capacity(8 + id.len() + self.payload.len());
        out.extend_from_slice(&(id.len() as u64).to_le_bytes());
        out.extend_from_slice(id);
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    fn deserialize(bytes: &[u8]) -> TranspileKindResult<Self>
    where
        Self: Sized,
    {
        let err = |m: &str| TranspileErrorKind::external(std::io::Error::other(m.to_string()));
        if bytes.len() < 8 {
            return Err(err("plugin artifact too short"));
        }
        let id_len = u64::from_le_bytes(bytes[..8].try_into().unwrap()) as usize;
        let end = 8usize
            .checked_add(id_len)
            .filter(|&e| e <= bytes.len())
            .ok_or_else(|| err("pllugin artifact truncated"))?;
        let plugin_pass_id =
            String::from_utf8(bytes[8..end].to_vec()).map_err(|_| err("plugin id not UTF-8"))?;
        Ok(Self {
            plugin_pass_id,
            payload: bytes[end..].to_vec(),
        })
    }
}
