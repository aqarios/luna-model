use std::collections::HashMap;

use lunamodel_error::LunaModelResult;
use lunamodel_serializer::prelude::{
    BytesDecodable, BytesEncodable, Creatable, Decodable, Decoder, Encodable, Version, Versioned,
};
use prost::Message;

use super::artifact::IntegerToBinaryArtifact;

#[derive(Message)]
struct SerVarMapEntry {
    #[prost(string, repeated, tag = 0)]
    keys: Vec<String>,
    #[prost(uint64, repeated, tag = 1)]
    vals: Vec<u64>,
}

impl From<&HashMap<String, usize>> for SerVarMapEntry {
    fn from(value: &HashMap<String, usize>) -> Self {
        let mut out = Self::default();
        for (k, v) in value.iter() {
            out.keys.push(k.clone());
            out.vals.push(*v as u64);
        }
        out
    }
}

impl Into<HashMap<String, usize>> for SerVarMapEntry {
    fn into(self) -> HashMap<String, usize> {
        self.keys
            .into_iter()
            .zip(self.vals.iter().map(|&v| v as usize))
            .collect()
    }
}

#[derive(Message)]
pub struct SerIntegerToBinaryArtifact {
    #[prost(string, repeated, tag = 0)]
    varmap_keys: Vec<String>,
    #[prost(message, repeated, tag = 1)]
    varmap_vals: Vec<SerVarMapEntry>,
    #[prost(message, repeated, tag = 2)]
    offsetmap_keys: Vec<String>,
    #[prost(uint64, repeated, tag = 3)]
    offsetmap_vals: Vec<u64>,
}

impl Creatable<IntegerToBinaryArtifact> for SerIntegerToBinaryArtifact {
    fn new(value: &IntegerToBinaryArtifact) -> Self {
        Self::default().fill(value)
    }
}

impl BytesEncodable for SerIntegerToBinaryArtifact {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<IntegerToBinaryArtifact> for SerIntegerToBinaryArtifact {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<IntegerToBinaryArtifact> {
        Self::decode(bytes)?.extract()
    }
}

impl SerIntegerToBinaryArtifact {
    fn fill(mut self, value: &IntegerToBinaryArtifact) -> Self {
        for (k, v) in value.varmap.iter() {
            self.varmap_keys.push(k.clone());
            self.varmap_vals.push(v.into());
        }
        for (k, v) in value.offsetmap.iter() {
            self.offsetmap_keys.push(k.clone());
            self.offsetmap_vals.push(*v as u64);
        }
        self
    }

    fn extract(self) -> LunaModelResult<IntegerToBinaryArtifact> {
        Ok(IntegerToBinaryArtifact {
            varmap: self
                .varmap_keys
                .into_iter()
                .zip(self.varmap_vals.into_iter().map(|v| v.into()))
                .collect(),
            offsetmap: self
                .offsetmap_keys
                .into_iter()
                .zip(self.offsetmap_vals.into_iter().map(|v| v as usize))
                .collect(),
        })
    }
}

impl Encodable<SerIntegerToBinaryArtifact> for IntegerToBinaryArtifact {
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<IntegerToBinaryArtifact> for Vec<u8> {
    type Latest = SerIntegerToBinaryArtifact;
    type Payload = ();
}

impl Decodable<IntegerToBinaryArtifact> for Versioned<Vec<u8>> {
    type Latest = SerIntegerToBinaryArtifact;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<IntegerToBinaryArtifact> {
        match self.version {
            Some(Version::V0) => SerIntegerToBinaryArtifact::decoder(self.data.as_slice(), payload),
            _ => SerIntegerToBinaryArtifact::decoder(self.data.as_slice(), payload),
        }
    }
}
