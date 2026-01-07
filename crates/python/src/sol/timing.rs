use derive_more::Deref;
use lunamodel_core::{Timer, Timing};
use pyo3::pyclass;

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
