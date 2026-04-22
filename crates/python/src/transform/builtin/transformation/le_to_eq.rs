use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{LeToEqConstraintsArtifact, LeToEqConstraintsPass};
use lunamodel_transpiler::Artifact;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

#[pyclass]
pub struct PyArtifact(pub LeToEqConstraintsArtifact);

#[pymethods]
impl PyArtifact {
    #[getter]
    fn slackvars(&self) -> Vec<String> {
        self.0.slackvars().to_vec()
    }

    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(LeToEqConstraintsArtifact::deserialize(
            buf.as_bytes(py),
        )?))
    }
}

#[pytransformation(PyArtifact)]
#[derive(Default)]
pub struct PyLeToEqConstraintsPass(pub LeToEqConstraintsPass);

#[pymethods]
impl PyLeToEqConstraintsPass {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}
