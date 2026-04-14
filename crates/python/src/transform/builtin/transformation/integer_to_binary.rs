use std::collections::HashMap;

use lunamodel_transform::transformation::{IntegerToBinaryArtifact, IntegerToBinaryPass};
use lunamodel_transpiler::{Artifact, ReversiblePass};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::{PyModel, PySolution, transform::PyPassContext};

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

#[pyclass(subclass)]
#[derive(Default)]
pub struct PyIntegerToBinaryPass(pub IntegerToBinaryPass);

#[pymethods]
impl PyIntegerToBinaryPass {
    #[new]
    fn new() -> Self {
        Self::default()
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

    fn forward(&self, model: PyModel, ctx: &PyPassContext) -> PyResult<(PyModel, PyArtifact)> {
        let mut model = model.m.read_arc().clone();
        let artifact = self.0.forward(&mut model, &ctx.into())?;
        Ok((model.into(), PyArtifact(artifact)))
    }

    #[classmethod]
    fn backward(
        _cls: &Bound<'_, PyType>,
        artifact: &PyArtifact,
        solution: PySolution,
    ) -> PyResult<PySolution> {
        Ok(IntegerToBinaryPass::backward(&artifact.0, solution.s.read_arc().clone())?.into())
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PyIntegerToBinaryPass {
    pub fn to_rs(&self) -> IntegerToBinaryPass {
        self.0.clone()
    }
}
