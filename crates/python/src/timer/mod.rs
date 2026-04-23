mod timer;
mod timing;

use derive_more::Deref;
use lunamodel_core::{Timer, Timing};
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
#[derive(Clone, Deref)]
pub struct PyTiming(pub Timing);

#[pyclass]
#[repr(C)]
pub struct PyTimer(pub Timer);

impl From<PyTiming> for Timing {
    fn from(val: PyTiming) -> Self {
        val.0
    }
}

impl From<PyTimer> for Timer {
    fn from(val: PyTimer) -> Self {
        val.0
    }
}

impl From<Timing> for PyTiming {
    fn from(value: Timing) -> Self {
        Self(value)
    }
}
