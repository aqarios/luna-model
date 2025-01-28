#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq, Eq)]
pub struct Quadratic {
    v: u8,
}

impl Quadratic {
    pub fn empty() -> Self {
        Self { v: 0 }
    }
}
