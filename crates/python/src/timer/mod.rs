//! Python wrappers for timing and timer utilities.
mod create;
mod timing;

use derive_more::Deref;
use lunamodel_core::{Timer, Timing};
use pyo3::pyclass;

#[pyclass(from_py_object)]
#[repr(C)]
#[derive(Clone, Deref)]
pub struct PyTiming(pub Timing);

#[pyclass]
#[repr(C)]
pub struct PyTimer(pub Timer);

impl From<PyTiming> for Timing {
    /// Unwraps Python timing into the core type.
    fn from(val: PyTiming) -> Self {
        val.0
    }
}

impl From<PyTimer> for Timer {
    /// Unwraps Python timer into the core type.
    fn from(val: PyTimer) -> Self {
        val.0
    }
}

impl From<Timing> for PyTiming {
    /// Wraps core timing for Python.
    fn from(value: Timing) -> Self {
        Self(value)
    }
}
