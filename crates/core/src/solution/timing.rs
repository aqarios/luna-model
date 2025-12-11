use std::time::{Duration, SystemTime};

use lunamodel_error::{LunaModelError, LunaModelResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timing {
    start: SystemTime,
    end: SystemTime,
    pub qpu: Option<f64>,
}

impl Timing {
    pub(crate) fn new(start: SystemTime, end: SystemTime, qpu: Option<f64>) -> Self {
        Self { start, end, qpu }
    }

    pub fn total(&self) -> LunaModelResult<Duration> {
        self.end
            .duration_since(self.start)
            .map_err(|e| LunaModelError::Internal(e.to_string().into()))
    }

    pub fn start(&self) -> SystemTime {
        self.start
    }

    pub fn end(&self) -> SystemTime {
        self.end
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
