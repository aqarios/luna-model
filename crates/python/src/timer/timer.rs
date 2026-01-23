use pyo3::pymethods;

use lunamodel_core::Timer;
use lunamodel_unwind::*;

use super::{PyTimer, PyTiming};

#[unwindable]
#[pymethods]
impl PyTimer {
    /// Create a timer that starts counting immediately.
    ///
    /// Returns
    /// -------
    /// Timer
    ///     The timer.
    #[staticmethod]
    fn start() -> Self {
        Self(Timer::start())
    }

    /// Stop the timer, and get the resulting `Timing` object.
    ///
    /// Returns
    /// -------
    /// Timing
    ///     The timing object that holds the start and end time.
    fn stop(&self) -> PyTiming {
        PyTiming(self.0.stop())
    }
}
