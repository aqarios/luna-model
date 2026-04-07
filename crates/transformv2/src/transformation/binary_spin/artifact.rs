use std::collections::HashMap;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::Artifact;
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
            other => Err(LunaModelError::Compilation(
                format!("BinarySpinPass: unsupported target vtype '{other}'").into(),
            )),
        }
    }
}

impl Artifact for BinarySpinPassArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::binary-spin"
    }

    fn serialize(&self) -> LunaModelResult<Vec<u8>> {
        self.encode(Some(true), Some(3))
    }

    fn deserialize(bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized,
    {
        bytes.unversionize().decompress()?.decode(())
    }
}
