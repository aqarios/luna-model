//! Serializable artifact helpers for `<=` to equality conversion.

use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{
    BytesDecodable, BytesEncodable, Creatable, Decodable, Decoder, Encodable, Version, Versioned,
};
use prost::Message;

use crate::transformation::LeToEqConstraintsArtifact;

#[derive(Message)]
pub struct SerLeToEqConstraintsArtifact {
    #[prost(string, repeated, tag = "1")]
    slackvars: Vec<String>,
}

impl Creatable<LeToEqConstraintsArtifact> for SerLeToEqConstraintsArtifact {
    /// Creates the protobuf representation from the runtime artifact.
    fn new(value: &LeToEqConstraintsArtifact) -> Self {
        Self {
            slackvars: value.slackvars.clone(),
        }
    }
}

impl BytesEncodable for SerLeToEqConstraintsArtifact {
    /// Encodes the protobuf structure into raw bytes.
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<LeToEqConstraintsArtifact> for SerLeToEqConstraintsArtifact {
    /// Decodes raw bytes into the runtime artifact.
    fn decode_from_bytes(bytes: &[u8], _: ()) -> LunaModelResult<LeToEqConstraintsArtifact> {
        Ok(LeToEqConstraintsArtifact {
            slackvars: Self::decode(bytes)?.slackvars,
        })
    }
}

impl Encodable<SerLeToEqConstraintsArtifact> for LeToEqConstraintsArtifact {
    /// Returns the serializer version used by this artifact.
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<LeToEqConstraintsArtifact> for Vec<u8> {
    type Latest = SerLeToEqConstraintsArtifact;
    type Payload = ();
}

impl Decodable<LeToEqConstraintsArtifact> for Versioned<Vec<u8>> {
    type Latest = SerLeToEqConstraintsArtifact;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<LeToEqConstraintsArtifact> {
        match self.version {
            Some(Version::V0) => {
                SerLeToEqConstraintsArtifact::decoder(self.data.as_slice(), payload)
            }
            _ => SerLeToEqConstraintsArtifact::decoder(self.data.as_slice(), payload),
        }
    }
}
