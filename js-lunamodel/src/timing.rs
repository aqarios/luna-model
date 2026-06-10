use std::time::{SystemTime, UNIX_EPOCH};

use lunamodel::core::Timing as CoreTiming;
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
    pub fn start(&self) -> Result<f64> {
        millis_since_epoch(self.inner.start())
    }

    /// Wall-clock end time, in milliseconds since the Unix epoch (UTC).
    ///
    /// This matches Python's `end` property; wrap with `new Date(...)` on
    /// the JS side if you want a `Date` object.
    #[napi(getter)]
    pub fn end(&self) -> Result<f64> {
        millis_since_epoch(self.inner.end())
    }

    /// Total runtime in seconds.
    ///
    /// This is computed as the difference between `end` and `start`. Throws if
    /// the timing record is inconsistent and the total duration cannot be
    /// computed. This matches Python's `total_seconds` property.
    #[napi(getter)]
    pub fn total_seconds(&self) -> Result<f64> {
        self.inner.total().map(|d| d.as_secs_f64()).map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("Solution timing could not be computed correctly. Reason: {err}"),
            )
        })
    }

    /// QPU usage time reported by the backend.
    ///
    /// Returns `null` when no QPU timing was provided. This matches Python's
    /// `qpu` property.
    #[napi(getter)]
    pub fn qpu(&self) -> Option<f64> {
        self.inner.qpu
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
    use std::time::Duration;

    use super::*;

    #[test]
    fn timestamps_return_millis_since_epoch() {
        let timing = JsTiming::from(CoreTiming::new(
            UNIX_EPOCH + Duration::from_secs(1),
            UNIX_EPOCH + Duration::from_secs(3),
            Some(0.25),
        ));

        assert_eq!(timing.start().unwrap(), 1000.0);
        assert_eq!(timing.end().unwrap(), 3000.0);
        assert_eq!(timing.total_seconds().unwrap(), 2.0);
        assert_eq!(timing.qpu(), Some(0.25));
    }

    #[test]
    fn total_seconds_reports_inconsistent_timestamps() {
        let timing = JsTiming::from(CoreTiming::new(
            UNIX_EPOCH + Duration::from_secs(3),
            UNIX_EPOCH + Duration::from_secs(1),
            None,
        ));

        let err = timing.total_seconds().unwrap_err();

        assert!(
            err.reason
                .contains("Solution timing could not be computed correctly")
        );
        assert_eq!(timing.qpu(), None);
    }
}
