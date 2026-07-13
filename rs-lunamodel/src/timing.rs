//! Owning wrapper over a LunaModel timing handle.
//!
//! Mirrors the Rust `Timing`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Timing` still return `Result` here.

use std::time::{SystemTime, UNIX_EPOCH};

use lunamodel_cext_vtable::LmTiming;
use rs_lunamodel_derive::RawHandle;

use crate::{Error, Result, Status};

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
