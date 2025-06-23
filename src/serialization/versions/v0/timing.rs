use crate::core::Timing;
use crate::serialization::encodable::{BytesDecodable, BytesEncodable};
use crate::serialization::DecodeError;
use prost::Message;
use std::ops::Add;
use std::time::{Duration, SystemTime};

#[derive(Clone, PartialEq, Message)]
pub struct SerTiming {
    #[prost(double, tag = 1)]
    start: f64,

    #[prost(double, tag = 2)]
    end: f64,

    #[prost(double, optional, tag = 3)]
    qpu: Option<f64>,
}

impl BytesEncodable for SerTiming {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl BytesDecodable<Timing> for SerTiming {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> Result<Timing, DecodeError> {
        Ok(Self::decode(bytes)?.extract())
    }
}

impl SerTiming {
    pub fn extract(&self) -> Timing {
        let start = SystemTime::UNIX_EPOCH.add(Duration::from_secs_f64(self.start));
        let end = SystemTime::UNIX_EPOCH.add(Duration::from_secs_f64(self.end));

        Timing {
            start,
            end,
            qpu: self.qpu,
        }
    }
}
