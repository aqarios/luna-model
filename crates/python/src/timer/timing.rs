//! Python wrappers around timing records.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::{
    PyResult,
    exceptions::{PyRuntimeError, PyValueError},
    pymethods,
};
use std::time::{Duration, SystemTime};

use super::PyTiming;

#[unwindable]
#[pymethods]
impl PyTiming {
    #[getter]
    fn start(&self) -> SystemTime {
        self.0.start()
    }

    #[getter]
    fn end(&self) -> SystemTime {
        self.0.end()
    }

    /// The difference of the end and start time.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If total cannot be computed due to an inconsistent start or end time.
    #[getter]
    fn get_total(&self) -> PyResult<Duration> {
        self.total().map_err(|e| {
            PyRuntimeError::new_err(format!(
                "Solution timing could not be computed correctly. Reason: {e}"
            ))
        })
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
        self.0.qpu
    }

    /// Set the qpu usage time.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If `value` is negative.
    #[setter]
    fn set_qpu(&mut self, value: Option<f64>) -> PyResult<()> {
        if value.unwrap_or_default() < 0.0 {
            Err(PyValueError::new_err("QPU time must not be negative."))
        } else {
            self.0.qpu = value;
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
    ///     If `value` is negative.
    fn add_qpu(&mut self, value: f64) -> PyResult<()> {
        if value < 0.0 {
            Err(PyValueError::new_err("QPU time must not be negative."))
        } else {
            self.0.qpu = Some(self.qpu.unwrap_or_default() + value);
            Ok(())
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __str__(&self) -> String {
        format!("{}", self.0.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0.format(FormatOpt::Py))
    }
}
