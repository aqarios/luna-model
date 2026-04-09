use lunamodel_transformv2::transformation::BinarySpinPass;
use pyo3::{pyclass, pymethods};

use crate::PyVtype;

#[pyclass(subclass)]
pub struct PyBinarySpinPass(pub BinarySpinPass);

#[pymethods]
impl PyBinarySpinPass {
    #[new]
    #[pyo3(signature = (vtype, prefix=None))]
    fn new(vtype: PyVtype, prefix: Option<String>) -> Self {
        Self(BinarySpinPass::new(vtype.into(), prefix))
    }

    #[getter]
    fn vtype(&self) -> PyVtype {
        self.0.vtype().into()
    }

    #[getter]
    fn prefix(&self) -> Option<String> {
        self.0.prefix()
    }
}

impl PyBinarySpinPass {
    pub fn to_rs(&self) -> BinarySpinPass {
        self.0.clone()
    }
}
