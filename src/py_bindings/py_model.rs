use crate::core::{Model, VarId};
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Model", subclass)]
#[derive(Deref, DerefMut)]
pub struct PyModel(pub Model<VarId, f64>);

#[pymethods]
impl PyModel {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
