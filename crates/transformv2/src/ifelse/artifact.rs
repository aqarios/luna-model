use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_transpiler::{Artifact, CompilationRecord};

#[derive(Clone, Copy)]
pub enum BranchTaken {
    Then,
    Else,
}

impl BranchTaken {
    fn to_u8(&self) -> u8 {
        match self {
            Self::Then => 0,
            Self::Else => 1,
        }
    }

    fn from_u8(b: u8) -> LunaModelResult<Self> {
        match b {
            0 => Ok(Self::Then),
            1 => Ok(Self::Else),
            _ => Err(LunaModelError::Decoding(
                format!("'{b}' does not encode a valid 'BranchTaken' option.").into(),
            )),
        }
    }
}

pub struct IfElseArtifact {
    pub(super) branch: BranchTaken,
    pub(super) branch_record: CompilationRecord,
}

impl Artifact for IfElseArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::if_else"
    }

    fn serialize(&self) -> LunaModelResult<Vec<u8>> {
        let mut blob = vec![self.branch.to_u8()];
        blob.append(&mut self.branch_record.encode(Some(true), Some(3))?);
        Ok(blob)
    }

    fn deserialize(bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized,
    {
        // first byte encodes the branch.
        let branch = BranchTaken::from_u8(bytes[0])?;
        let branch_record: CompilationRecord =
            (&bytes[1..]).unversionize().decompress()?.decode(())?;
        Ok(Self {
            branch,

            branch_record,
        })
    }
}
