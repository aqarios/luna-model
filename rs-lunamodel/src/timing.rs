//! Owning wrapper over a LunaModel timing handle.
//!
//! Mirrors the Rust `Timing`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Timing` still return `Result` here.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lunamodel_cext_vtable::LmTiming;
use rs_lunamodel_derive::RawHandle;

use crate::{Error, Result, Status, TryClone};

/// Owning handle to a LunaModel timing (frees on drop).
#[derive(RawHandle)]
pub struct Timing {
    handle: *mut LmTiming,
}

impl Drop for Timing {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, timing_free)
        }
    }
}

impl Timing {
    pub fn new(start: SystemTime, end: SystemTime, qpu: Option<f64>) -> Result<Self> {
        out!(timing_new(unix_nanos(start, "start")?, unix_nanos(end, "end")?, ?qpu))
    }

    pub fn total(&self) -> Result<Duration> {
        Ok(Duration::from_nanos(out!(timing_total_ns(@self))?))
    }

    pub fn start(&self) -> Result<SystemTime> {
        Ok(UNIX_EPOCH + Duration::from_nanos(out!(timing_start_ns(@self))?))
    }

    pub fn end(&self) -> Result<SystemTime> {
        Ok(UNIX_EPOCH + Duration::from_nanos(out!(timing_end_ns(@self))?))
    }

    pub fn qpu(&self) -> Result<Option<f64>> {
        out_opt!(timing_qpu(@self))
    }
}

impl TryClone for Timing {
    fn try_clone(&self) -> Result<Self> {
        Self::new(self.start()?, self.end()?, self.qpu()?)
    }
}

impl PartialEq for Timing {
    fn eq(&self, other: &Self) -> bool {
        self.start().ok() == other.start().ok()
            && self.end().ok() == other.end().ok()
            && self.qpu().ok() == other.qpu().ok()
    }
}

impl std::fmt::Debug for Timing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timing")
            .field("start", &self.start().ok())
            .field("end", &self.end().ok())
            .field("qpu", &self.qpu().ok().flatten())
            .finish()
    }
}

fn unix_nanos(time: SystemTime, field: &str) -> Result<u64> {
    let duration = time.duration_since(UNIX_EPOCH).map_err(|_| Error {
        status: Status::InvalidArgument,
        message: format!("{field} must not precede the Unix epoch"),
    })?;

    u64::try_from(duration.as_nanos()).map_err(|_| Error {
        status: Status::InvalidArgument,
        message: format!("{field} is too large to represent as Unix nanoseconds"),
    })
}
