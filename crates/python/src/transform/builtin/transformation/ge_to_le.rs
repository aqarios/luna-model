//! Python wrapper for `>=` to `<=` constraint normalization.

use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{GeToLeConstraintsArtifact, GeToLeConstraintsPass};
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
pub struct PyArtifact(pub GeToLeConstraintsArtifact);

#[pymethods]
impl PyArtifact {
    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize().map_err(to_pyerr)?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(
            GeToLeConstraintsArtifact::deserialize(buf.as_bytes(py)).map_err(to_pyerr)?,
        ))
    }
}

#[pytransformation(PyArtifact)]
#[derive(Default)]
pub struct PyGeToLeConstraintsPass(pub GeToLeConstraintsPass);

#[pymethods]
impl PyGeToLeConstraintsPass {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}
