use lunamodel_transpiler::TransformationOutput;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use crate::{
    PyModel, PySolution,
    transform::{PyPassContext, record::PyTransformationRecord},
};

#[pyclass]
#[repr(C)]
pub struct PyTransformationOutput {
    pub to: TransformationOutput,
}

impl From<TransformationOutput> for PyTransformationOutput {
    fn from(to: TransformationOutput) -> Self {
        Self { to }
    }
}

impl PyTransformationOutput {
    pub fn backward(&self, solution: PySolution) -> PyResult<PySolution> {
        Ok(self
            .to
            .record
            .backward(solution.s.read_arc().clone())?
            .into())
    }
}

#[unwindable]
#[pymethods]
impl PyTransformationOutput {
    #[getter]
    fn model(&self) -> PyModel {
        self.to.model.clone().into()
    }

    #[getter]
    fn record(&self) -> PyTransformationRecord {
        self.to.record.clone().into()
    }

    #[getter]
    fn context(&self) -> PyPassContext {
        self.to.analysis.clone().into()
    }
}
