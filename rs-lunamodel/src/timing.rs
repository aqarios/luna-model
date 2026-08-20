//! Owning wrapper over a LunaModel timing handle.
//!
//! Mirrors the Rust `Timing`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Timing` still return `Result` here.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lunamodel_capi::LmTiming;
use rs_lunamodel_derive::RawHandle;

use crate::{Result, TryClone, api::api};

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
    /// Creates a timing with the given total duration in seconds and no
    /// recorded start/end time or sub-timings.
    pub fn new(total: f64) -> Result<Self> {
        out!(timing_new(total))
    }

    /// The total time in seconds an algorithm needed to run.
    pub fn total(&self) -> Result<f64> {
        out!(timing_total_seconds(@self))
    }

    /// The recorded start time, if any.
    pub fn start(&self) -> Result<Option<SystemTime>> {
        out_opt!(timing_start_ns(@self), |ns: u64| Ok(UNIX_EPOCH + Duration::from_nanos(ns)))
    }

    /// The recorded end time, if any.
    pub fn end(&self) -> Result<Option<SystemTime>> {
        out_opt!(timing_end_ns(@self), |ns: u64| Ok(UNIX_EPOCH + Duration::from_nanos(ns)))
    }

    /// The values recorded under `key`, or an empty vector if absent.
    pub fn get(&self, key: &str) -> Result<Vec<f64>> {
        let key = crate::traits::TryToCString::try_to_cstring(key)?;
        let mut buffer = std::mem::MaybeUninit::uninit();
        crate::error::check(unsafe {
            (api().timing_get)(self.as_ptr(), key.as_ptr(), buffer.as_mut_ptr())
        })?;
        let buffer = unsafe { buffer.assume_init() };
        let values = if buffer.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(buffer.data, buffer.len) }.to_vec()
        };
        unsafe { (api().f64_buffer_free)(buffer) };
        Ok(values)
    }

    /// The sum of the values recorded under `key`, or `None` if absent.
    pub fn total_for(&self, key: &str) -> Result<Option<f64>> {
        out_opt!(timing_total_for(@self, !key))
    }

    /// Overwrites the values recorded under `key` with a single `value`.
    pub fn set(&mut self, key: &str, value: f64) -> Result<()> {
        check!(timing_set(@mut self, !key, value))
    }

    /// Appends `value` to the list recorded under `key`.
    pub fn add(&mut self, key: &str, value: f64) -> Result<()> {
        check!(timing_add(@mut self, !key, value))
    }

    pub fn merge(&mut self, other: &Self) -> Result<()> {
        check!(timing_merge(@mut self, @other))
    }
}

impl TryClone for Timing {
    fn try_clone(&self) -> Result<Self> {
        out!(timing_clone(@self))
    }
}

impl PartialEq for Timing {
    fn eq(&self, other: &Self) -> bool {
        out_or!(timing_eq(@self, @other), false)
    }
}

impl std::fmt::Debug for Timing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timing")
            .field("total", &self.total().ok())
            .field("start", &self.start().ok().flatten())
            .field("end", &self.end().ok().flatten())
            .finish()
    }
}
