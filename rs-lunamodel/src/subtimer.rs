//! Owning wrapper over a LunaModel sub-timer handle.
//!
//! Mirrors the Rust `SubTimer`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table.

use lunamodel_capi::LmSubTimer;
use rs_lunamodel_derive::RawHandle;

use crate::Result;

/// Owning handle to a LunaModel sub-timer.
///
/// Created by [`crate::Timer::record`]. Dropping it without calling [`SubTimer::stop`]
/// discards the measurement unrecorded.
#[derive(RawHandle)]
pub struct SubTimer {
    handle: *mut LmSubTimer,
}

impl Drop for SubTimer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, subtimer_free)
        }
    }
}

impl SubTimer {
    /// Stops the sub-timer, recording its elapsed time (in seconds) on the
    /// parent timer under the key it was started with, and returns that
    /// elapsed time. Consumes the sub-timer.
    pub fn stop(mut self) -> Result<f64> {
        let result = out!(subtimer_stop(@mut self));
        // The ABI call already consumed and freed the handle; skip the double free in `Drop`.
        self.handle = std::ptr::null_mut();
        result
    }
}
