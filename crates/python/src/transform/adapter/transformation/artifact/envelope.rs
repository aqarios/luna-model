//! Common wrapper trait for Python transformation artifacts.

use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::transform::adapter::transformation::envelope::BackwardEnvelope;
use crate::transform::envelope::{Envelope, EnvelopeUtils};

const MAGIC: [u8; 4] = *b"PTA1";
const VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub struct ArtifactEnvelope {
    pub(super) module: String,
    pub(super) qualname: String,
    pub(super) content: Vec<u8>,
    pub(super) backward: BackwardEnvelope,
}

impl Envelope for ArtifactEnvelope {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(MAGIC);
        out.push(VERSION);
        Self::write_field(&mut out, self.module.as_bytes());
        Self::write_field(&mut out, self.qualname.as_bytes());
        Self::write_field(&mut out, &self.content);
        Self::write_field(&mut out, &self.backward.encode());
        out
    }

    fn decode(bytes: &[u8]) -> LunaModelResult<Self> {
        let mut i = 0usize;
        if bytes.len() < 5 || bytes[0..4] != MAGIC {
            return Err(LunaModelError::Decoding("invalid artifact header".into()));
        }
        i += 4;

        let version = bytes[i];
        i += 1;

        if version != VERSION {
            return Err(LunaModelError::Decoding(
                format!("unsupported artifact version: {version}").into(),
            ));
        }

        let module = Self::read_string(bytes, &mut i)?;
        let qualname = Self::read_string(bytes, &mut i)?;
        let content = Self::read_bytes(bytes, &mut i)?;
        let backward = Self::read_bytes(bytes, &mut i)?;

        Ok(Self {
            module,
            qualname,
            content,
            backward: BackwardEnvelope::decode(&backward)?,
        })
    }
}
