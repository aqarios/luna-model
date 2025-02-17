use crate::core::Model;

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Model")]
#[derive(Deref, DerefMut)]
pub struct PyModel(pub Model);

#[pymethods]
impl PyModel {}
