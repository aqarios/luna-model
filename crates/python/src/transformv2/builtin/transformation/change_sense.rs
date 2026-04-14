use lunamodel_transformv2::transformation::{ChangeSensePass, ChangeSensePassArtifact};
use lunamodel_transpiler::{Artifact, ReversiblePass};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::{PyModel, PySense, PySolution, transformv2::PyPassContext};

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

#[pyclass(subclass)]
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
    ) -> PyResult<(PyModel, PyChangeSensePassArtifact)> {
        let mut model = model.m.read_arc().clone();
        let artifact = self.0.forward(&mut model, &ctx.into())?;
        Ok((model.into(), PyChangeSensePassArtifact(artifact)))
    }

    #[classmethod]
    fn backward(
        _cls: &Bound<'_, PyType>,
        artifact: &PyChangeSensePassArtifact,
        solution: PySolution,
    ) -> PyResult<PySolution> {
        Ok(ChangeSensePass::backward(&artifact.0, solution.s.read_arc().clone())?.into())
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PyChangeSensePass {
    pub fn to_rs(&self) -> ChangeSensePass {
        self.0.clone()
    }
}
