use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{
    ReduceInvertedBinaryPass, ReduceInvertedBinaryPassArtifact,
};
use lunamodel_transpiler::Artifact;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

#[pyclass]
pub struct PyReduceInvertedBinaryPassArtifact(pub ReduceInvertedBinaryPassArtifact);

#[pymethods]
impl PyReduceInvertedBinaryPassArtifact {
    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(ReduceInvertedBinaryPassArtifact::deserialize(
            buf.as_bytes(py),
        )?))
    }
}

#[pytransformation(PyReduceInvertedBinaryPassArtifact)]
pub struct PyReduceInvertedBinaryPass(pub ReduceInvertedBinaryPass);

#[pymethods]
impl PyReduceInvertedBinaryPass {
    #[new]
    fn new() -> Self {
        Self(ReduceInvertedBinaryPass::new())
    }
}
