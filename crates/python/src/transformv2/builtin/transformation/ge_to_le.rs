use lunamodel_transformv2::transformation::GeToLeConstraintsPass;
use pyo3::{pyclass, pymethods};

#[pyclass(subclass)]
#[derive(Default)]
pub struct PyGeToLeConstraintsPass(pub GeToLeConstraintsPass);

#[pymethods]
impl PyGeToLeConstraintsPass {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

impl PyGeToLeConstraintsPass {
    pub fn to_rs(&self) -> GeToLeConstraintsPass {
        self.0.clone()
    }
}
