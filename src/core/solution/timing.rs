use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timing {
    /// The time at which the algorithm started.
    pub start: SystemTime,
    /// The solver's or algorithm's total runtime in seconds.
    pub end: SystemTime,
    /// The qpu usage time in seconds.
    pub qpu: Option<f64>,
}

impl Timing {
    pub fn new(start: SystemTime, end: SystemTime, qpu: Option<f64>) -> Self {
        Self { start, end, qpu }
    }

    pub fn total(&self) -> Result<Duration, std::time::SystemTimeError> {
        self.end.duration_since(self.start)
    }
}

pub struct Timer(SystemTime);

impl Timer {
    pub fn start() -> Self {
        Self(SystemTime::now())
    }

    pub fn stop(&self) -> Timing {
        let end = SystemTime::now();
        Timing::new(self.0, end, None)
    }
}
