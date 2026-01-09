use derive_more::Deref;
use lunamodel_core::{Timer, Timing};
use pyo3::{pyclass, pymethods};

#[pyclass]
#[repr(C)]
#[derive(Clone, Deref)]
pub struct PyTiming(pub Timing);

#[pyclass]
pub struct PyTimer(pub Timer);

impl Into<Timing> for PyTiming {
    fn into(self) -> Timing {
        self.0
    }
}

impl Into<Timer> for PyTimer {
    fn into(self) -> Timer {
        self.0
    }
}

impl From<Timing> for PyTiming {
    fn from(value: Timing) -> Self {
        Self(value)
    }
}

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
