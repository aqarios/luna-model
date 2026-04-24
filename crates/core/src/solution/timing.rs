use std::time::{Duration, SystemTime};

use lunamodel_error::{LunaModelError, LunaModelResult};

/// Timing metadata attached to a solution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timing {
    /// Wall-clock start time.
    start: SystemTime,
    /// Wall-clock end time.
    end: SystemTime,
    /// Optional QPU/device execution time reported by the backend.
    pub qpu: Option<f64>,
}

impl Timing {
    /// Creates a timing record from explicit timestamps.
    pub fn new(start: SystemTime, end: SystemTime, qpu: Option<f64>) -> Self {
        Self { start, end, qpu }
    }

    /// Returns the total wall-clock duration.
    pub fn total(&self) -> LunaModelResult<Duration> {
        self.end
            .duration_since(self.start)
            .map_err(|e| LunaModelError::Internal(e.to_string().into()))
    }

    /// Returns the start timestamp.
    pub fn start(&self) -> SystemTime {
        self.start
    }

    /// Returns the end timestamp.
    pub fn end(&self) -> SystemTime {
        self.end
    }
}

/// Lightweight stopwatch used while constructing solution metadata.
pub struct Timer(SystemTime);

impl Timer {
    /// Starts a new wall-clock timer at `SystemTime::now()`.
    pub fn start() -> Self {
        Self(SystemTime::now())
    }

    /// Stops the timer and creates a [`Timing`] record without QPU metadata.
    pub fn stop(&self) -> Timing {
        let end = SystemTime::now();
        Timing::new(self.0, end, None)
    }
}
