//! Base trait and wrapper plumbing for Python transformation artifacts.

use lunamodel_error::{LunaModelError, LunaModelResult};

pub trait Envelope {
    fn encode(&self) -> Vec<u8>;

    fn decode(bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized;
}

pub trait EnvelopeUtils {
    fn write_field(out: &mut Vec<u8>, data: &[u8]) {
        out.extend((data.len() as u32).to_be_bytes());
        out.extend(data);
    }

    fn read_u32(bytes: &[u8], i: &mut usize) -> LunaModelResult<usize> {
        if *i + 4 > bytes.len() {
            return Err(LunaModelError::Decoding(
                "truncated artifact field length".into(),
            ));
        }
        let len =
            u32::from_be_bytes([bytes[*i], bytes[*i + 1], bytes[*i + 2], bytes[*i + 3]]) as usize;
        *i += 4;
        Ok(len)
    }

    fn read_bytes(bytes: &[u8], i: &mut usize) -> LunaModelResult<Vec<u8>> {
        let len = Self::read_u32(bytes, i)?;
        if *i + len > bytes.len() {
            return Err(LunaModelError::Decoding(
                "truncated artifact field content".into(),
            ));
        }
        let out = bytes[*i..*i + len].to_vec();
        *i += len;
        Ok(out)
    }

    fn read_string(bytes: &[u8], i: &mut usize) -> LunaModelResult<String> {
        let raw = Self::read_bytes(bytes, i)?;
        String::from_utf8(raw).map_err(|e| LunaModelError::Decoding(e.to_string().into()))
    }
}

impl<P: Envelope> EnvelopeUtils for P {}
