use lunamodel_transpiler::TransformationOutput;
use prost::Message;

use crate::encode::{BytesDecodable, BytesEncodable, Creatable, Decodable, Encodable};

#[derive(Clone, PartialEq, Message)]
pub struct SerIR {
    #[prost(bytes, tag = "1")]
    record: Vec<u8>,
    #[prost(bytes, tag = "2")]
    model: Vec<u8>,
}

impl Creatable<TransformationOutput> for SerIR {
    fn new(to: &TransformationOutput) -> Self {
        Self {
            record: to.record.serialize(),
            model: to.model.serialize(),
        }
    }
}

impl BytesEncodable for SerIR {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<TransformationOutput> for SerIR {
    fn decode_from_bytes(
        bytes: &[u8],
        _: (),
    ) -> lunamodel_error::LunaModelResult<TransformationOutput> {
        let ser = Self::decode(bytes)?;
        Ok(TransformationOutput {
            record: ser.record.decode(())?,
            model: ser.model.decode(())?,
        })
    }
}
