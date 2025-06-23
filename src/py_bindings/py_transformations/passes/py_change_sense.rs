use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

use crate::{
    core::Sense,
    transformations::{
        base_passes::{BasePass, Pass},
        passes::change_sense::ChangeSensePass,
    },
};

use super::py_pass_base::PyPass;

// TODO: Docs
#[pyclass(name = "ChangeSensePass")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyChangeSensePass(ChangeSensePass);

#[pymethods]
impl PyChangeSensePass {
    #[new]
    #[pyo3(signature=(sense=Sense::Min))]
    pub fn py_init(sense: Sense) -> Self {
        PyChangeSensePass(ChangeSensePass { sense })
    }

    #[getter]
    pub fn get_sense(&self) -> Sense {
        self.sense
    }

    #[getter]
    pub fn get_name(&self) -> String {
        self.name().to_owned()
    }

    #[getter]
    pub fn get_requires(&self) -> Vec<String> {
        self.requires()
    }
}

impl PyPass for PyChangeSensePass {
    fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::Transformation(Box::new(self.0)))
    }
}
