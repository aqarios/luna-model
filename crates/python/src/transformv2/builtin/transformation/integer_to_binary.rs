use lunamodel_transformv2::transformation::IntegerToBinaryPass;
use pyo3::{pyclass, pymethods};

#[pyclass(subclass)]
#[derive(Default)]
pub struct PyIntegerToBinaryPass(pub IntegerToBinaryPass);

#[pymethods]
impl PyIntegerToBinaryPass {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

impl PyIntegerToBinaryPass {
    pub fn to_rs(&self) -> IntegerToBinaryPass {
        self.0.clone()
    }
}
