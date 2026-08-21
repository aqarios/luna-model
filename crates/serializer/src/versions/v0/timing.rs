//! Version 0 timing encoding.

use lunamodel_core::Timing;
use lunamodel_error::LunaModelResult;
use prost::Message;
use std::ops::Add;
use std::time::{Duration, UNIX_EPOCH};

use crate::encode::{BytesDecodable, BytesEncodable, Creatable};

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
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<Timing> {
        Ok(Self::decode(bytes)?.extract())
    }
}

impl Creatable<Timing> for SerTiming {
    fn new(value: &Timing) -> Self {
        Self::default().fill(value)
    }
}

impl SerTiming {
    fn fill(mut self, timing: &Timing) -> Self {
        self.start = timing
            .start()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.end = timing
            .end()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.qpu = timing.qpu;

        self
    }

    pub fn extract(&self) -> Timing {
        let start = UNIX_EPOCH.add(Duration::from_secs_f64(self.start));
        let end = UNIX_EPOCH.add(Duration::from_secs_f64(self.end));
        Timing::new(start, end, self.qpu)
    }
}
