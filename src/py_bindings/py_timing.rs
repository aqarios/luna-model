use crate::core::{Timer, Timing};
use derive_more::{Deref, DerefMut};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::{pyclass, pymethods, PyResult};
use std::time::{Duration, SystemTime};

#[pyclass(unsendable, name = "Timing")]
#[derive(Clone, Deref, DerefMut, Debug)]
pub struct PyTiming(pub Timing);

#[pyclass(unsendable, name = "Timer")]
#[derive(Deref, DerefMut)]
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

#[pymethods]
impl PyTiming {
    #[getter]
    fn start(&self) -> SystemTime {
        self.start
    }

    #[getter]
    fn end(&self) -> SystemTime {
        self.end
    }

    #[getter(total)]
    fn get_total(&self) -> PyResult<Duration> {
        self.total()
            .map_err(|_| PyRuntimeError::new_err("Solution timing could not be computed correctly"))
    }

    #[getter]
    fn total_seconds(&self) -> PyResult<f64> {
        self.get_total().map(|t| t.as_secs_f64())
    }

    #[getter]
    fn qpu(&self) -> Option<f64> {
        self.qpu
    }

    #[setter(qpu)]
    fn set_qpu(&mut self, value: Option<f64>) -> PyResult<()> {
        if value.unwrap_or_default() < 0f64 {
            Err(PyValueError::new_err("QPU time must not be negative."))
        } else {
            self.qpu = value;
            Ok(())
        }
    }

    fn add_qpu(&mut self, value: f64) -> PyResult<()> {
        if value < 0f64 {
            Err(PyValueError::new_err("QPU time must not be negative."))
        } else {
            self.qpu = Some(self.qpu.unwrap_or_default() + value);
            Ok(())
        }
    }
}

#[pymethods]
impl PyTimer {
    #[staticmethod]
    fn start() -> Self {
        Self(Timer::start())
    }

    fn stop(&self) -> PyTiming {
        PyTiming(self.0.stop())
    }
}
