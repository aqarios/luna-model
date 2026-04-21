use lunamodel_transform::transformation::{
    ReduceInvertedBinaryPass, ReduceInvertedBinaryPassArtifact,
};
use lunamodel_transpiler::{Artifact, Reversible, TransformationPass};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::{PyModel, PySolution, transform::PyPassContext};

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

#[pyclass(subclass)]
pub struct PyReduceInvertedBinaryPass(pub ReduceInvertedBinaryPass);

#[pymethods]
impl PyReduceInvertedBinaryPass {
    #[new]
    fn new() -> Self {
        Self(ReduceInvertedBinaryPass::new())
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
    ) -> PyResult<(PyModel, PyReduceInvertedBinaryPassArtifact)> {
        let mut model = model.m.read_arc().clone();
        let artifact = self.0.forward(&mut model, &ctx.into())?;
        Ok((model.into(), PyReduceInvertedBinaryPassArtifact(artifact)))
    }

    #[classmethod]
    fn backward(
        _cls: &Bound<'_, PyType>,
        artifact: &PyReduceInvertedBinaryPassArtifact,
        solution: PySolution,
    ) -> PyResult<PySolution> {
        Ok(ReduceInvertedBinaryPass::backward(&artifact.0, solution.s.read_arc().clone())?.into())
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PyReduceInvertedBinaryPass {
    pub fn to_rs(&self) -> ReduceInvertedBinaryPass {
        self.0.clone()
    }
}
