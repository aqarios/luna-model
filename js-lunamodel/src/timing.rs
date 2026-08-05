use std::time::{SystemTime, UNIX_EPOCH};

use luna_model::core::Timing as CoreTiming;
use napi::bindgen_prelude::{Error, Result, Status};
use napi_derive::napi;

/// Runtime timing metadata attached to a solution.
///
/// JavaScript exposes wall-clock timestamps as milliseconds since the Unix
/// epoch (UTC). Wrap with `new Date(timing.start)` for a `Date` object.
#[napi(js_name = "Timing")]
pub struct JsTiming {
    inner: CoreTiming,
}

#[napi]
impl JsTiming {
    /// Wall-clock start time, in milliseconds since the Unix epoch (UTC).
    ///
    /// This matches Python's `start` property; wrap with `new Date(...)` on
    /// the JS side if you want a `Date` object.
    #[napi(getter)]
    pub fn start(&self) -> Result<Option<f64>> {
        self.inner.start.map(|t| millis_since_epoch(t)).transpose()
    }

    /// Wall-clock end time, in milliseconds since the Unix epoch (UTC).
    ///
    /// This matches Python's `end` property; wrap with `new Date(...)` on
    /// the JS side if you want a `Date` object.
    #[napi(getter)]
    pub fn end(&self) -> Result<Option<f64>> {
        self.inner.end.map(|t| millis_since_epoch(t)).transpose()
    }

    /// Total runtime in seconds.
    #[napi(getter)]
    pub fn total(&self) -> f64 {
        self.inner.total
    }

    /// QPU usage time reported by the backend.
    ///
    /// Returns `null` when no QPU timing was provided. This matches Python's
    /// `qpu` property.
    #[napi(getter)]
    pub fn qpu(&self) -> Option<f64> {
        self.inner.total_for("qpu")
    }
}

impl From<CoreTiming> for JsTiming {
    fn from(inner: CoreTiming) -> Self {
        Self { inner }
    }
}

fn millis_since_epoch(t: SystemTime) -> Result<f64> {
    t.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("timestamp predates the Unix epoch: {err}"),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_return_millis_since_epoch() {
        let mut coretiming = CoreTiming::new(2.0);
        coretiming.set("qpu", 0.25);
        let timing = JsTiming::from(coretiming);

        assert_eq!(timing.total(), 2.0);
        assert_eq!(timing.qpu(), Some(0.25));
    }
}
