use lunamodel_transformv2::transformation::LeToEqConstraintsPass;
use pyo3::{pyclass, pymethods};

#[pyclass(subclass)]
#[derive(Default)]
pub struct PyLeToEqConstraintsPass(pub LeToEqConstraintsPass);

#[pymethods]
impl PyLeToEqConstraintsPass {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

impl PyLeToEqConstraintsPass {
    pub fn to_rs(&self) -> LeToEqConstraintsPass {
        self.0.clone()
    }
}

