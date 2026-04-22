use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{ChangeSensePass, ChangeSensePassArtifact};
use lunamodel_transpiler::Artifact;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::PySense;

#[pyclass]
pub struct PyChangeSensePassArtifact(pub ChangeSensePassArtifact);

#[pymethods]
impl PyChangeSensePassArtifact {
    #[getter]
    fn did_change(&self) -> bool {
        self.0.did_change()
    }

    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(ChangeSensePassArtifact::deserialize(buf.as_bytes(py))?))
    }
}

#[pytransformation(PyChangeSensePassArtifact)]
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
