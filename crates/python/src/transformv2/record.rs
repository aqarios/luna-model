use lunamodel_transpiler::TransformationRecord;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use crate::{PySolution, transformv2::entry::PyPassEntry};

#[pyclass]
#[derive(Clone)]
pub struct PyTransformationRecord {
    pub tr: TransformationRecord,
}

impl From<TransformationRecord> for PyTransformationRecord {
    fn from(tr: TransformationRecord) -> Self {
        Self { tr }
    }
}

#[unwindable]
#[pymethods]
impl PyTransformationRecord {
    #[getter]
    fn entries(&self) -> Vec<PyPassEntry> {
        self.tr.entries().map(|e| e.into()).collect()
    }

    fn backward(&self, solution: PySolution) -> PyResult<PySolution> {
        Ok(self.tr.backward(solution.s.read_arc().clone())?.into())
    }
}
