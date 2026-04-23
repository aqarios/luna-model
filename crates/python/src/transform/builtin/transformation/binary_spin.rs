use std::collections::HashMap;

use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{BinarySpinPass, BinarySpinPassArtifact};
use lunamodel_transpiler::Artifact;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::PyVtype;

#[pyclass]
pub struct PyBinarySpinPassArtifact(pub BinarySpinPassArtifact);

#[pymethods]
impl PyBinarySpinPassArtifact {
    #[getter]
    fn map(&self) -> HashMap<String, String> {
        self.0.map().clone()
    }

    #[getter]
    fn old_vtype(&self) -> PyVtype {
        self.0.old_vtype().into()
    }

    #[getter]
    fn new_vtype(&self) -> PyVtype {
        self.0.new_vtype().into()
    }

    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(BinarySpinPassArtifact::deserialize(buf.as_bytes(py))?))
    }
}

#[pytransformation(PyBinarySpinPassArtifact)]
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
