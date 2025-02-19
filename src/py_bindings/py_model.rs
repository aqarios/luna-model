use crate::core::{Model, VarId};

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Model")]
#[derive(Deref, DerefMut)]
pub struct PyModel(pub Model<VarId, f64>);

#[pymethods]
impl PyModel {}
