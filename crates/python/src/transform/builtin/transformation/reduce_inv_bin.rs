//! Python wrapper for inverted-binary reduction.

use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{
    ReduceInvertedBinaryPass, ReduceInvertedBinaryPassArtifact,
};
use lunamodel_transpiler::{Artifact, Reversible, TransformationPass};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::{
    PyModel, PySolution,
    transform::{PyPassContext, error::to_pyerr},
};

#[pyclass]
pub struct PyReduceInvertedBinaryPassArtifact(pub ReduceInvertedBinaryPassArtifact);

#[pymethods]
impl PyReduceInvertedBinaryPassArtifact {
    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize().map_err(to_pyerr)?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(
            ReduceInvertedBinaryPassArtifact::deserialize(buf.as_bytes(py)).map_err(to_pyerr)?,
        ))
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
