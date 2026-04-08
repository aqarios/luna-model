use lunamodel_transpiler::TransformationRecord;
use lunamodel_unwind::unwindable;
use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct PyTransformationRecord {
    pub tr: TransformationRecord,
}

#[unwindable]
#[pymethods]
impl PyTransformationRecord {}

impl From<TransformationRecord> for PyTransformationRecord {
    fn from(tr: TransformationRecord) -> Self {
        Self { tr }
    }
}
