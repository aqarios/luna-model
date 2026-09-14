//! Python wrappers for timing and timer utilities.
mod create;
mod timing;

use derive_more::Deref;
use lunamodel_core::{SubTimer, Timer, Timing};
use parking_lot::RwLock;
use pyo3::pyclass;
use std::sync::Arc;

#[pyclass(from_py_object)]
#[repr(C)]
#[derive(Clone, Deref)]
pub struct PyTiming(pub Arc<RwLock<Timing>>);

#[pyclass]
#[repr(C)]
pub struct PyTimer(pub Timer);

#[pyclass]
pub struct PySubTimer(pub Option<SubTimer>);

impl From<PyTimer> for Timer {
    /// Unwraps Python timer into the core type.
    fn from(val: PyTimer) -> Self {
        val.0
    }
}

impl From<SubTimer> for PySubTimer {
    /// Unwraps Python timer into the core type.
    fn from(val: SubTimer) -> Self {
        Self(Some(val))
    }
}

impl From<Timing> for PyTiming {
    fn from(value: Timing) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

impl From<PyTiming> for Timing {
    fn from(value: PyTiming) -> Self {
        value.read_arc().clone()
    }
}
