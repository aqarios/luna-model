use lunamodel_transformv2::transformation::ChangeSensePass;
use pyo3::{pyclass, pymethods};

use crate::PySense;

#[pyclass(subclass)]
pub struct PyChangeSensePass(pub ChangeSensePass);

#[pymethods]
impl PyChangeSensePass {
    #[new]
    fn new(sense: PySense) -> Self {
        Self(ChangeSensePass::new(sense.into()))
    }

    #[getter]
    fn sense(&self) -> PySense {
        self.0.sense().into()
    }
}

impl PyChangeSensePass {
    pub fn to_rs(&self) -> ChangeSensePass {
        self.0.clone()
    }
}
