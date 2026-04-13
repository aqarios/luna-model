use lunamodel_transformv2::transformation::{
    EqualityConstraintsToQuadraticPenaltyArtifact, EqualityConstraintsToQuadraticPenaltyPass,
};
use lunamodel_transpiler::{Artifact, ReversiblePass};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::{PyModel, PySolution, transformv2::PyPassContext};

#[pyclass]
struct PyEqualityConstraintsToQuadraticPenaltyArtifact(
    EqualityConstraintsToQuadraticPenaltyArtifact,
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

#[pyclass(subclass)]
pub struct PyEqualityConstraintsToQuadraticPenaltyPass(
    pub EqualityConstraintsToQuadraticPenaltyPass,
);

#[pymethods]
impl PyEqualityConstraintsToQuadraticPenaltyPass {
    #[new]
    fn new(penalty_scaling: f64) -> Self {
        Self(EqualityConstraintsToQuadraticPenaltyPass::new(
            penalty_scaling,
        ))
    }

    #[getter]
    fn penalty_scaling(&self) -> f64 {
        self.0.penalty_scaling()
    }

    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().to_vec()
    }

    fn invalidates(&self) -> Vec<String> {
        self.0.invalidates().to_vec()
    }

    fn forward(
        &self,
        model: PyModel,
        ctx: &PyPassContext,
    ) -> PyResult<(PyModel, PyEqualityConstraintsToQuadraticPenaltyArtifact)> {
        let mut model = model.m.read_arc().clone();
        let artifact = self.0.forward(&mut model, &ctx.into())?;
        Ok((
            model.into(),
            PyEqualityConstraintsToQuadraticPenaltyArtifact(artifact),
        ))
    }

    #[classmethod]
    fn backward(
        _cls: &Bound<'_, PyType>,
        artifact: &PyEqualityConstraintsToQuadraticPenaltyArtifact,
        solution: PySolution,
    ) -> PyResult<PySolution> {
        Ok(EqualityConstraintsToQuadraticPenaltyPass::backward(
            &artifact.0,
            solution.s.read_arc().clone(),
        )?
        .into())
    }
}

impl PyEqualityConstraintsToQuadraticPenaltyPass {
    pub fn to_rs(&self) -> EqualityConstraintsToQuadraticPenaltyPass {
        self.0.clone()
    }
}
