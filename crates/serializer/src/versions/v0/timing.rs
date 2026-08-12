//! Version 0 timing encoding.

use lunamodel_core::Timing;
use lunamodel_error::{LunaModelError, LunaModelResult};
use prost::Message;
use std::ops::Add;
use std::time::{Duration, UNIX_EPOCH};

use crate::encode::BytesDecodable;

#[derive(Clone, PartialEq, Message)]
pub struct SerTiming {
    #[prost(double, tag = 1)]
    start: f64,

    #[prost(double, tag = 2)]
    end: f64,

    #[prost(double, optional, tag = 3)]
    qpu: Option<f64>,
}

impl BytesDecodable<Timing> for SerTiming {
    fn decode_from_bytes(bytes: &[u8], _payload: ()) -> LunaModelResult<Timing> {
        Self::decode(bytes)?.extract()
    }
}

impl SerTiming {
    fn extract(self) -> LunaModelResult<Timing> {
        let start = UNIX_EPOCH.add(Duration::from_secs_f64(self.start));
        let end = UNIX_EPOCH.add(Duration::from_secs_f64(self.end));
        let total = end
            .duration_since(start)
            .map_err(|e| LunaModelError::Internal(e.to_string().into()))?
            .as_secs_f64();
        let mut timing = Timing::new(total);
        timing.start = Some(start);
        timing.end = Some(end);
        if let Some(qpu) = self.qpu {
            timing.set("qpu", qpu);
        }
        Ok(timing)
    }
}
