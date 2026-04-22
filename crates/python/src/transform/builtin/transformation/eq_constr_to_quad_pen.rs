use lunamodel_python_macros::pytransformation;
use lunamodel_transform::transformation::{
    EqualityConstraintsToQuadraticPenaltyArtifact, EqualityConstraintsToQuadraticPenaltyPass,
};
use lunamodel_transpiler::Artifact;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

#[pyclass]
pub struct PyEqualityConstraintsToQuadraticPenaltyArtifact(
    pub EqualityConstraintsToQuadraticPenaltyArtifact,
);

#[pymethods]
impl PyEqualityConstraintsToQuadraticPenaltyArtifact {
    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(
            EqualityConstraintsToQuadraticPenaltyArtifact::deserialize(buf.as_bytes(py))?,
        ))
    }
}

#[pytransformation(PyEqualityConstraintsToQuadraticPenaltyArtifact)]
pub struct PyEqualityConstraintsToQuadraticPenaltyPass(
    pub EqualityConstraintsToQuadraticPenaltyPass,
);

#[pymethods]
impl PyEqualityConstraintsToQuadraticPenaltyPass {
    #[new]
    fn new(penalty_scaling: f64) -> Self {
        Self(EqualityConstraintsToQuadraticPenaltyPass::new(penalty_scaling))
    }

    #[getter]
    fn penalty_scaling(&self) -> f64 {
        self.0.penalty_scaling()
    }
}
