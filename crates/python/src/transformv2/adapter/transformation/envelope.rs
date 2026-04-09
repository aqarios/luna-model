use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::transformv2::envelope::{Envelope, EnvelopeUtils};

const MAGIC: [u8; 4] = *b"PTB1";
const VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub struct BackwardEnvelope {
    pub(super) module: String,
    pub(super) qualname: String,
}

impl Envelope for BackwardEnvelope {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(MAGIC);
        out.push(VERSION);
        Self::write_field(&mut out, self.module.as_bytes());
        Self::write_field(&mut out, self.qualname.as_bytes());
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
        Ok(Self { module, qualname })
    }
}
