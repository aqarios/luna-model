use chrono::{DateTime, Utc};
use lunamodel_core::Timing as CoreTiming;
use napi::bindgen_prelude::{Error, Result, Status};
use napi_derive::napi;

/// Runtime timing metadata attached to a solution.
///
/// JavaScript exposes wall-clock timestamps as `Date` objects and durations as
/// numeric seconds.
#[napi(js_name = "Timing")]
pub struct JsTiming {
    inner: CoreTiming,
}

#[napi]
impl JsTiming {
    /// Wall-clock start time.
    ///
    /// This matches Python's `start` property.
    #[napi(getter)]
    pub fn start(&self) -> DateTime<Utc> {
        self.inner.start().into()
    }

    /// Wall-clock end time.
    ///
    /// This matches Python's `end` property.
    #[napi(getter)]
    pub fn end(&self) -> DateTime<Utc> {
        self.inner.end().into()
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

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use super::*;

    #[test]
    fn total_seconds_returns_duration_in_seconds() {
        let timing = JsTiming::from(CoreTiming::new(
            UNIX_EPOCH + Duration::from_secs(1),
            UNIX_EPOCH + Duration::from_secs(3),
            Some(0.25),
        ));

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
