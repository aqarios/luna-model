use crate::core::{Timer, Timing};
use derive_more::{Deref, DerefMut};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::{pyclass, pymethods, PyResult};
use std::time::{Duration, SystemTime};

/// The object that holds information about an algorithm's runtime.
///
/// This class can only be constructed using a `Timer`. This ensures that a
/// `Timing` object always contains a start as well as an end time.
///
/// The `qpu` field of this class can only be set after constructing it with a timer.
///
/// Examples
/// --------
/// >>> from dwave.samplers.tree.solve import BinaryQuadraticModel
/// >>> from luna_quantum import Model, Timer, Timing
/// >>> model = ... # third-party model
/// >>> algorithm = ... # third-party algorithm
/// >>> timer = Timer.start()
/// >>> sol = algorithm.run(model)
/// >>> timing: Timing = timer.stop()
/// >>> timing.qpu = sol.qpu_time
/// >>> timing.total_seconds
/// 1.2999193
/// >>> timing.qpu
/// 0.02491934
#[pyclass(unsendable, name = "Timing")]
#[derive(Clone, Deref, DerefMut, Debug)]
pub struct PyTiming(pub Timing);

/// Used to measure the computation time of an algorithm.
///
/// The sole purpose of the ``Timer`` class is to create a ``Timing`` object in a safe
/// way, i.e., to ensure that the ``Timing`` object always holds a starting and
/// finishing time.
///
/// Examples
/// --------
/// Basic usage:
/// >>> from luna_quantum import Timer
/// >>> timer = Timer.start()
/// >>> solution = ... # create a solution by running an algorithm.
/// >>> timing = timer.stop()
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
    /// The starting time of the algorithm.
    #[getter]
    fn start(&self) -> SystemTime {
        self.start
    }

    /// The end, or finishing, time of the algorithm.
    #[getter]
    fn end(&self) -> SystemTime {
        self.end
    }

    /// The difference of the end and start time.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If total cannot be computed due to an inconsistent start or end time.
    #[getter]
    fn get_total(&self) -> PyResult<Duration> {
        self.total()
            .map_err(|_| PyRuntimeError::new_err("Solution timing could not be computed correctly"))
    }

    /// The total time in seconds an algorithm needed to run. Computed as the
    /// difference of end and start time.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If total_seconds cannot be computed due to an inconsistent start or end time.
    #[getter]
    fn total_seconds(&self) -> PyResult<f64> {
        self.get_total().map(|t| t.as_secs_f64())
    }

    /// The qpu usage time of the algorithm this timing object was created for.
    #[getter]
    fn qpu(&self) -> Option<f64> {
        self.qpu
    }

    /// Set the qpu usage time.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If ``value`` is negative.
    #[setter]
    fn set_qpu(&mut self, value: Option<f64>) -> PyResult<()> {
        if value.unwrap_or_default() < 0.0 {
            Err(PyValueError::new_err("QPU time must not be negative."))
        } else {
            self.qpu = value;
            Ok(())
        }
    }

    /// Add qpu usage time to the qpu usage time already present. If the current value
    /// is None, this method acts like a setter.
    ///
    /// Parameters
    /// ----------
    /// value : float
    ///     The value to add to the already present qpu value.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If ``value`` is negative.
    fn add_qpu(&mut self, value: f64) -> PyResult<()> {
        if value < 0.0 {
            Err(PyValueError::new_err("QPU time must not be negative."))
        } else {
            self.qpu = Some(self.qpu.unwrap_or_default() + value);
            Ok(())
        }
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

    /// Stop the timer, and get the resulting ``Timing`` object.
    ///
    /// Returns
    /// -------
    /// Timing
    ///     The timing object that holds the start and end time.
    fn stop(&self) -> PyTiming {
        PyTiming(self.0.stop())
    }
}
