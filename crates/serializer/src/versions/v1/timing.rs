//! Version 0 timing encoding.

use lunamodel_core::Timing;
use lunamodel_error::LunaModelResult;
use prost::Message;
use std::collections::HashMap;
use std::ops::Add;
use std::time::{Duration, UNIX_EPOCH};

use crate::encode::{BytesDecodable, BytesEncodable, Creatable};

#[derive(Clone, PartialEq, Message)]
struct DoubleList {
    #[prost(double, repeated, tag = 1)]
    data: Vec<f64>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SerTiming {
    #[prost(double, tag = 1)]
    total: f64,
    #[prost(map = "string, message", tag = 2)]
    timings: HashMap<String, DoubleList>,
    #[prost(double, optional, tag = 3)]
    maybe_start: Option<f64>,
    #[prost(double, optional, tag = 4)]
    maybe_end: Option<f64>,
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
        self.maybe_start = timing
            .start
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_secs_f64());
        self.maybe_end = timing
            .end
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_secs_f64());

        self.total = timing.total;
        self.timings = timing
            .timings
            .iter()
            .map(|(key, values)| {
                (
                    key.clone(),
                    DoubleList {
                        data: values.clone(),
                    },
                )
            })
            .collect();
        self
    }

    pub fn extract(self) -> Timing {
        Timing {
            start: self
                .maybe_start
                .map(|t| UNIX_EPOCH.add(Duration::from_secs_f64(t))),
            end: self
                .maybe_end
                .map(|t| UNIX_EPOCH.add(Duration::from_secs_f64(t))),
            total: self.total,
            timings: self
                .timings
                .into_iter()
                .map(|(key, data)| (key, data.data))
                .collect(),
        }
    }
}
