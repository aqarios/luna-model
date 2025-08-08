use std::fmt::Debug;

use pyo3::prelude::*;

#[pyclass(subclass, name = "BasePass")]
#[derive(Debug)]
pub struct PyBasePass {}
