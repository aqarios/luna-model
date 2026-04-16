use std::collections::HashMap;

use lunamodel_transform::transformation::{BinarySpinPass, BinarySpinPassArtifact};
use lunamodel_transpiler::{Artifact, Reversible, TransformationPass};
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyBytes, PyType},
};

use crate::{PyModel, PySolution, PyVtype, transform::PyPassContext};

#[pyclass]
pub struct PyBinarySpinPassArtifact(pub BinarySpinPassArtifact);

#[pymethods]
impl PyBinarySpinPassArtifact {
    #[getter]
    fn map(&self) -> HashMap<String, String> {
        self.0.map().clone()
    }

    #[getter]
    fn old_vtype(&self) -> PyVtype {
        self.0.old_vtype().into()
    }

    #[getter]
    fn new_vtype(&self) -> PyVtype {
        self.0.new_vtype().into()
    }

    fn serialize(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, self.0.serialize()?.as_slice()).into())
    }

    #[classmethod]
    fn deserialize(_cls: &Bound<'_, PyType>, py: Python, buf: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self(BinarySpinPassArtifact::deserialize(buf.as_bytes(py))?))
    }
}

#[pyclass(subclass)]
pub struct PyBinarySpinPass(pub BinarySpinPass);

#[pymethods]
impl PyBinarySpinPass {
    #[new]
    #[pyo3(signature = (vtype, prefix=None))]
    fn new(vtype: PyVtype, prefix: Option<String>) -> Self {
        Self(BinarySpinPass::new(vtype.into(), prefix))
    }

    #[getter]
    fn vtype(&self) -> PyVtype {
        self.0.vtype().into()
    }

    #[getter]
    fn prefix(&self) -> Option<String> {
        self.0.prefix()
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
    ) -> PyResult<(PyModel, PyBinarySpinPassArtifact)> {
        let mut model = model.m.read_arc().clone();
        let artifact = self.0.forward(&mut model, &ctx.into())?;
        Ok((model.into(), PyBinarySpinPassArtifact(artifact)))
    }

    #[classmethod]
    fn backward(
        _cls: &Bound<'_, PyType>,
        artifact: &PyBinarySpinPassArtifact,
        solution: PySolution,
    ) -> PyResult<PySolution> {
        Ok(BinarySpinPass::backward(&artifact.0, solution.s.read_arc().clone())?.into())
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PyBinarySpinPass {
    pub fn to_rs(&self) -> BinarySpinPass {
        self.0.clone()
    }
}
