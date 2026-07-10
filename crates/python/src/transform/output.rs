//! Python wrapper for transformation results.

use lunamodel_transpiler::TransformationOutput;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use crate::{
    PyModel, PySolution,
    transform::{PyPassContext, error::to_pyerr, record::PyTransformationRecord},
};

#[pyclass]
#[repr(C)]
pub struct PyTransformationOutput {
    /// The underlying Rust transformation result.
    pub to: TransformationOutput,
}

impl From<TransformationOutput> for PyTransformationOutput {
    fn from(to: TransformationOutput) -> Self {
        Self { to }
    }
}

impl PyTransformationOutput {
    /// Run the recorded reverse transformation sequence on a solution.
    pub fn backward(&self, solution: PySolution) -> PyResult<PySolution> {
        Ok(self
            .to
            .record
            .backward(solution.s.read_arc().clone())
            .map_err(to_pyerr)?
            .into())
    }
}

#[unwindable]
#[pymethods]
impl PyTransformationOutput {
    /// Return the transformed model.
    #[getter]
    fn model(&self) -> PyModel {
        self.to.model.clone().into()
    }

    /// Return the transformation record used for backwards mapping.
    #[getter]
    fn record(&self) -> PyTransformationRecord {
        self.to.record.clone().into()
    }

    /// Return the analysis context accumulated while running the pipeline.
    #[getter]
    fn context(&self) -> PyPassContext {
        self.to.analysis.clone().into()
    }
}
