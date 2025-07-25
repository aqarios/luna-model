use std::fmt::Debug;

use pyo3::prelude::*;

#[pyclass(unsendable, subclass, name = "BasePass")]
#[derive(Debug)]
pub struct PyBasePass {}
