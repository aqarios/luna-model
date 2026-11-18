//! Owning wrapper over a LunaModel timer handle.
//!
//! Mirrors the Rust `Timer`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Timer` still return `Result` here.

use luna_model_ffi_derive::RawHandle;
use lunamodel_capi::LmTimer;

use crate::{Result, SubTimer, Timing, api::api};

/// Owning handle to a LunaModel timer (frees on drop).
#[derive(RawHandle)]
pub struct Timer {
    handle: *mut LmTimer,
}

impl Drop for Timer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, timer_free)
        }
    }
}

impl Timer {
    pub fn start() -> Self {
        Self::from_raw((api().timer_start)())
    }

    /// Starts a fine-grained sub-timer under `key`, to be stopped via
    /// [`SubTimer::stop`].
    pub fn record(&self, key: &str) -> Result<SubTimer> {
        out!(timer_record(@self, !key))
    }

    /// Stops the timer, producing its accumulated [`Timing`]. The timer
    /// remains valid and reusable after this call.
    pub fn stop(&self) -> Result<Timing> {
        out!(timer_stop(@self))
    }
}
