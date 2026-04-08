use lunamodel_transpiler::{ErasedArtifact, PassEntry};
use pyo3::pyclass;

use crate::transformv2::record::PyTransformationRecord;

#[pyclass(get_all)]
#[derive(Clone)]
pub struct PyErasedArtifact {
    type_tag: String,
    data: Vec<u8>,
}

impl From<&ErasedArtifact> for PyErasedArtifact {
    fn from(value: &ErasedArtifact) -> Self {
        Self {
            type_tag: value.type_tag().to_string(),
            data: value.data().clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum PyPassEntry {
    Transform {
        pass_id: String,
        pass_name: String,
        artifact: PyErasedArtifact,
    },

    /// An analysis pass (no artifact, not reversed)
    Analysis { pass_name: String },

    /// A nested sub-pipeline
    Pipeline {
        name: String,
        record: PyTransformationRecord,
    },

    /// A nested sub-pipeline originating from a ControlFlow
    ControlFlow {
        pass_name: String,
        name: String,
        record: PyTransformationRecord,
    },
}

impl From<&PassEntry> for PyPassEntry {
    fn from(value: &PassEntry) -> Self {
        match value {
            PassEntry::Transform {
                pass_id,
                pass_name,
                artifact,
            } => Self::Transform {
                pass_id: pass_id.clone(),
                pass_name: pass_name.clone(),
                artifact: artifact.into(),
            },
            PassEntry::Analysis { pass_name } => Self::Analysis {
                pass_name: pass_name.clone(),
            },
            PassEntry::ControlFlow {
                pass_name,
                name,
                record,
            } => Self::ControlFlow {
                pass_name: pass_name.clone(),
                name: name.clone(),
                record: record.clone().into(),
            },
            PassEntry::Pipeline { name, record } => Self::Pipeline {
                name: name.clone(),
                record: record.clone().into(),
            },
        }
    }
}
