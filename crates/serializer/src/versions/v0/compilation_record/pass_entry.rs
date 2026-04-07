use std::str::FromStr;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::PassEntry;
use prost::Message;
use strum_macros::{Display, EnumString};

use crate::{
    encode::{BytesDecodable, BytesEncodable, Creatable},
    versions::v0::compilation_record::{SerCompilationRecord, artifact::SerErasedArtifact},
};

#[derive(EnumString, Display)]
enum PassEntryType {
    #[strum(to_string = "PassEntry::Transform")]
    T,
    #[strum(to_string = "PassEntry::Analysis")]
    A,
    #[strum(to_string = "PassEntry::Pipeline")]
    P,
}

#[derive(Clone, PartialEq, Message)]
pub struct SerPassEntry {
    #[prost(string, tag = "1")]
    entry_type: String,
    #[prost(string, optional, tag = "2")]
    id: Option<String>,
    #[prost(string, tag = "3")]
    name: String,
    #[prost(bytes, optional, tag = "4")]
    content: Option<Vec<u8>>,
}

impl BytesEncodable for SerPassEntry {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl From<&PassEntry> for SerPassEntry {
    fn from(value: &PassEntry) -> Self {
        match value {
            PassEntry::Transform {
                pass_id,
                pass_name,
                artifact,
            } => Self {
                entry_type: PassEntryType::T.to_string(),
                id: Some(pass_id.to_string()),
                name: pass_name.to_string(),
                content: Some(SerErasedArtifact::from(artifact).encode_to_bytes()),
            },
            PassEntry::Analysis { pass_name } => Self {
                entry_type: PassEntryType::A.to_string(),
                name: pass_name.to_string(),
                ..Default::default()
            },
            PassEntry::Pipeline { name, record } => Self {
                entry_type: PassEntryType::P.to_string(),
                name: name.into(),
                content: Some(SerCompilationRecord::new(record.as_ref()).encode_to_bytes()),
                ..Default::default()
            },
        }
    }
}

impl SerPassEntry {
    pub fn extract(&self) -> LunaModelResult<PassEntry> {
        Ok(match PassEntryType::from_str(&self.entry_type)? {
            PassEntryType::T => PassEntry::Transform {
                pass_name: self.name.clone(),
                pass_id: self.id.as_ref().expect("id was not serialized").to_string(),
                artifact: SerErasedArtifact::decode_from_bytes(
                    self.content
                        .as_ref()
                        .expect("artifact was not serialized")
                        .as_slice(),
                    (),
                )?,
            },
            PassEntryType::A => PassEntry::Analysis {
                pass_name: self.name.clone(),
            },
            PassEntryType::P => PassEntry::Pipeline {
                name: self.name.clone(),
                record: Box::new(SerCompilationRecord::decode_from_bytes(
                    self.content
                        .as_ref()
                        .expect("record was not serialized")
                        .as_slice(),
                    (),
                )?),
            },
        })
    }
}
