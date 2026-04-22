use std::collections::HashMap;

use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{IntegerToBinaryArtifact, IntegerToBinaryPass};
use lunamodel_transpiler::Artifact;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

#[pyclass]
pub struct PyArtifact(pub IntegerToBinaryArtifact);

#[pymethods]
impl PyArtifact {
    #[getter]
    fn varmap(&self) -> HashMap<String, HashMap<String, usize>> {
        self.0.varmap().clone()
    }

    #[getter]
    fn offsetmap(&self) -> HashMap<String, usize> {
        self.0.offsetmap().clone()
    }

    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(IntegerToBinaryArtifact::deserialize(
            buf.as_bytes(py),
        )?))
    }
}

#[pytransformation(PyArtifact)]
#[derive(Default)]
pub struct PyIntegerToBinaryPass(pub IntegerToBinaryPass);

#[pymethods]
impl PyIntegerToBinaryPass {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}
