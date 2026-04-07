use std::str::FromStr;

use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{
    BytesDecodable, BytesEncodable, Creatable, Decodable, Decoder, Encodable, Version, Versioned,
};
use lunamodel_types::Vtype;
use prost::Message;

use super::artifact::BinarySpinPassArtifact;

#[derive(Message)]
pub struct SerBinSpinArtifact {
    #[prost(string, repeated, tag = "1")]
    keys: Vec<String>,
    #[prost(string, repeated, tag = "2")]
    vals: Vec<String>,
    #[prost(string, tag = "3")]
    old_vtype: String,
    #[prost(string, tag = "4")]
    new_vtype: String,
}

impl Creatable<BinarySpinPassArtifact> for SerBinSpinArtifact {
    fn new(value: &BinarySpinPassArtifact) -> Self {
        Self::default().fill(value)
    }
}

impl BytesEncodable for SerBinSpinArtifact {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<BinarySpinPassArtifact> for SerBinSpinArtifact {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<BinarySpinPassArtifact> {
        Self::decode(bytes)?.extract()
    }
}

impl SerBinSpinArtifact {
    fn fill(mut self, value: &BinarySpinPassArtifact) -> Self {
        self.new_vtype = value.new_vtype.to_string();
        self.old_vtype = value.old_vtype.to_string();
        for (k, v) in value.map.iter() {
            self.keys.push(k.clone());
            self.vals.push(v.clone());
        }
        self
    }

    fn extract(self) -> LunaModelResult<BinarySpinPassArtifact> {
        Ok(BinarySpinPassArtifact {
            new_vtype: Vtype::from_str(&self.new_vtype)?,
            old_vtype: Vtype::from_str(&self.old_vtype)?,
            map: self.keys.into_iter().zip(self.vals).collect(),
        })
    }
}

impl Encodable<SerBinSpinArtifact> for BinarySpinPassArtifact {
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<BinarySpinPassArtifact> for Vec<u8> {
    type Latest = SerBinSpinArtifact;
    type Payload = ();
}

impl Decodable<BinarySpinPassArtifact> for Versioned<Vec<u8>> {
    type Latest = SerBinSpinArtifact;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<BinarySpinPassArtifact> {
        match self.version {
            Some(Version::V0) => SerBinSpinArtifact::decoder(self.data.as_slice(), payload),
            _ => SerBinSpinArtifact::decoder(self.data.as_slice(), payload),
        }
    }
}
