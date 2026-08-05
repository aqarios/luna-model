//! Constructors for Python timer utilities.

use parking_lot::RwLock;
use pyo3::{PyResult, pymethods};
use std::sync::Arc;

use lunamodel_core::Timer;
use lunamodel_unwind::*;

use crate::timer::PySubTimer;

use super::{PyTimer, PyTiming};

#[unwindable]
#[pymethods]
impl PyTimer {
    #[new]
    fn new() -> Self {
        Self(Timer::new())
    }

    /// Create a timer that starts counting immediately.
    ///
    /// Returns
    /// -------
    /// Timer
    ///     The timer.
    #[staticmethod]
    fn start() -> Self {
        Self(Timer::new())
    }

    fn record(&self, timing: String) -> PyResult<PySubTimer> {
        Ok(self.0.start(timing).into())
    }

    /// Stop the timer, and get the resulting `Timing` object.
    ///
    /// Returns
    /// -------
    /// Timing
    ///     The timing object that holds the start and end time.
    fn stop(&self) -> PyResult<PyTiming> {
        Ok(PyTiming(Arc::new(RwLock::new(self.0.clone().stop()?))))
    }
}

#[unwindable]
#[pymethods]
impl PySubTimer {
    fn stop(&mut self) -> PyResult<f64> {
        let timer = self.0.take().ok_or_else(|| {
            lunamodel_error::LunaModelError::Internal("timer already stopped".into())
        })?;
        Ok(timer.stop()?)
    }
}
