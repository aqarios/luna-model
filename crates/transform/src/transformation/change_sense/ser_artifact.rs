//! Serializable artifact helpers for objective-sense normalization.

use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{
    BytesDecodable, BytesEncodable, Creatable, Decodable, Decoder, Encodable, Version, Versioned,
};
use prost::Message;

use super::artifact::ChangeSensePassArtifact;

#[derive(Message)]
pub struct SerChangeSenseArtifact {
    #[prost(bool, tag = "1")]
    did_change: bool,
}

impl Creatable<ChangeSensePassArtifact> for SerChangeSenseArtifact {
    fn new(value: &ChangeSensePassArtifact) -> Self {
        Self::default().fill(value)
    }
}

impl BytesEncodable for SerChangeSenseArtifact {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<ChangeSensePassArtifact> for SerChangeSenseArtifact {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<ChangeSensePassArtifact> {
        Self::decode(bytes)?.extract()
    }
}

impl SerChangeSenseArtifact {
    fn fill(mut self, value: &ChangeSensePassArtifact) -> Self {
        self.did_change = value.did_change;
        self
    }

    fn extract(self) -> LunaModelResult<ChangeSensePassArtifact> {
        Ok(ChangeSensePassArtifact {
            did_change: self.did_change,
        })
    }
}

impl Encodable<SerChangeSenseArtifact> for ChangeSensePassArtifact {
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<ChangeSensePassArtifact> for Vec<u8> {
    type Latest = SerChangeSenseArtifact;
    type Payload = ();
}

impl Decodable<ChangeSensePassArtifact> for Versioned<Vec<u8>> {
    type Latest = SerChangeSenseArtifact;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<ChangeSensePassArtifact> {
        match self.version {
            Some(Version::V0) => SerChangeSenseArtifact::decoder(self.data.as_slice(), payload),
            _ => SerChangeSenseArtifact::decoder(self.data.as_slice(), payload),
        }
    }
}
