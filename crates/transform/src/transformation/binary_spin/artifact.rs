//! Artifact types for binary-to-spin conversion.

use std::collections::HashMap;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::{Artifact, TranspileErrorKind, TranspileKindResult};
use lunamodel_types::Vtype;

pub struct BinarySpinPassArtifact {
    pub(super) map: HashMap<String, String>,
    pub(super) old_vtype: Vtype,
    pub(super) new_vtype: Vtype,
}

impl BinarySpinPassArtifact {
    pub(super) fn try_new(vtype: Vtype) -> LunaModelResult<Self> {
        match vtype {
            Vtype::Spin => Ok(Self {
                map: HashMap::new(),
                old_vtype: Vtype::Binary,
                new_vtype: Vtype::Spin,
            }),
            Vtype::Binary => Ok(Self {
                map: HashMap::new(),
                old_vtype: Vtype::Spin,
                new_vtype: Vtype::Binary,
            }),
            other => Err(LunaModelError::transformation(format!(
                "BinarySpinPass: unsupported target vtype '{other}'"
            ))),
        }
    }

    pub fn map(&self) -> &HashMap<String, String> {
        &self.map
    }

    pub fn old_vtype(&self) -> Vtype {
        self.old_vtype
    }

    pub fn new_vtype(&self) -> Vtype {
        self.new_vtype
    }
}

impl Artifact for BinarySpinPassArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "luna_model::binary-spin"
    }

    fn serialize(&self) -> TranspileKindResult<Vec<u8>> {
        Ok(self.encode(Some(true), Some(3))?)
    }

    fn deserialize(bytes: &[u8]) -> TranspileKindResult<Self>
    where
        Self: Sized,
    {
        Ok(bytes
            .unversionize()
            .decompress()
            .map_err(TranspileErrorKind::external)?
            .decode(())?)
    }
}
