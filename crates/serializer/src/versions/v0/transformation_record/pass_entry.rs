use std::str::FromStr;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::PassEntry;
use prost::Message;
use strum_macros::{Display, EnumString};

use crate::{
    encode::{BytesDecodable, BytesEncodable, Creatable},
    versions::v0::transformation_record::{SerTransformationRecord, artifact::SerErasedArtifact},
};

#[derive(EnumString, Display)]
enum PassEntryType {
    #[strum(to_string = "PassEntry::Transform")]
    T,
    #[strum(to_string = "PassEntry::Analysis")]
    A,
    #[strum(to_string = "PassEntry::Pipeline")]
    P,
    #[strum(to_string = "PassEntry::Composite")]
    C,
    #[strum(to_string = "PassEntry::ControlFlow")]
    CF,
    #[strum(to_string = "PassEntry::MetaAnalysis")]
    MA,
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
            PassEntry::Composite {
                pass_id,
                pass_name,
                artifact,
            } => Self {
                entry_type: PassEntryType::C.to_string(),
                id: Some(pass_id.to_string()),
                name: pass_name.to_string(),
                content: Some(SerErasedArtifact::from(artifact).encode_to_bytes()),
            },
            PassEntry::Analysis { pass_name } => Self {
                entry_type: PassEntryType::A.to_string(),
                name: pass_name.to_string(),
                ..Default::default()
            },
            PassEntry::MetaAnalysis { pass_name } => Self {
                entry_type: PassEntryType::MA.to_string(),
                name: pass_name.to_string(),
                ..Default::default()
            },
            PassEntry::Pipeline { name, record } => Self {
                entry_type: PassEntryType::P.to_string(),
                name: name.into(),
                content: Some(SerTransformationRecord::new(record).encode_to_bytes()),
                ..Default::default()
            },
            PassEntry::ControlFlow {
                pass_name,
                name,
                record,
            } => Self {
                entry_type: PassEntryType::CF.to_string(),
                id: Some(pass_name.to_string()),
                name: name.into(),
                content: Some(SerTransformationRecord::new(record).encode_to_bytes()),
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
            PassEntryType::C => PassEntry::Composite {
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
            PassEntryType::MA => PassEntry::MetaAnalysis {
                pass_name: self.name.clone(),
            },
            PassEntryType::P => PassEntry::Pipeline {
                name: self.name.clone(),
                record: SerTransformationRecord::decode_from_bytes(
                    self.content
                        .as_ref()
                        .expect("record was not serialized")
                        .as_slice(),
                    (),
                )?,
            },
            PassEntryType::CF => PassEntry::ControlFlow {
                pass_name: self.name.clone(),
                name: self
                    .id
                    .as_ref()
                    .expect("name was not serialized")
                    .to_string(),
                record: SerTransformationRecord::decode_from_bytes(
                    self.content
                        .as_ref()
                        .expect("record was not serialized")
                        .as_slice(),
                    (),
                )?,
            },
        })
    }
}
