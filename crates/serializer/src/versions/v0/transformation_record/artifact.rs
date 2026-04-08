use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::ErasedArtifact;
use prost::Message;

use crate::encode::{BytesDecodable, BytesEncodable};

#[derive(Clone, PartialEq, Message)]
pub struct SerErasedArtifact {
    #[prost(string, tag = "1")]
    type_tag: String,
    #[prost(bytes, tag = "2")]
    data: Vec<u8>,
}

impl From<&ErasedArtifact> for SerErasedArtifact {
    fn from(value: &ErasedArtifact) -> Self {
        Self {
            type_tag: value.type_tag().to_string(),
            data: value.data().clone(),
        }
    }
}

impl BytesEncodable for SerErasedArtifact {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<ErasedArtifact> for SerErasedArtifact {
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<ErasedArtifact> {
        let ser = Self::decode(bytes)?;
        Ok(ErasedArtifact::create(ser.type_tag, ser.data))
    }
}
