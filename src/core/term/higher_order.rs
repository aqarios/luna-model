use std::u8;

#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone)]
pub struct HigherOrder {
    v: u8,
}

impl HigherOrder {
    pub fn empty() -> Self {
        Self { v: 0 }
    }
}
