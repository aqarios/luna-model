use lunamodel_error::LunaModelError;
use lunamodel_transform::transformation::{
    BinarySpinPassArtifact, ChangeSensePassArtifact, EqualityConstraintsToQuadraticPenaltyArtifact,
    GeToLeConstraintsArtifact, IntegerToBinaryArtifact, LeToEqConstraintsArtifact,
    ReduceInvertedBinaryPassArtifact,
};
use lunamodel_transpiler::{ErasedArtifact, PassEntry};
use pyo3::{IntoPyObjectExt, Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::transform::{
    adapter::PyTransformationPassAdapterArtifact,
    builtin::transformation::{
        PyBinarySpinPassArtifact, PyChangeSensePassArtifact,
        PyEqualityConstraintsToQuadraticPenaltyArtifact, PyGeToLeConstraintsArtifact,
        PyIntegerToBinaryArtifact, PyLeToEqConstraintsArtifact, PyReduceInvertedBinaryPassArtifact,
    },
    record::PyTransformationRecord,
};

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

#[pymethods]
impl PyErasedArtifact {
    fn restore(&self, py: Python) -> PyResult<Py<PyAny>> {
        let ea = ErasedArtifact::create(self.type_tag.clone(), self.data.clone());
        if let Ok(b) = ea.restore::<BinarySpinPassArtifact>() {
            PyBinarySpinPassArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<ChangeSensePassArtifact>() {
            PyChangeSensePassArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<EqualityConstraintsToQuadraticPenaltyArtifact>() {
            PyEqualityConstraintsToQuadraticPenaltyArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<GeToLeConstraintsArtifact>() {
            PyGeToLeConstraintsArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<IntegerToBinaryArtifact>() {
            PyIntegerToBinaryArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<LeToEqConstraintsArtifact>() {
            PyLeToEqConstraintsArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<ReduceInvertedBinaryPassArtifact>() {
            PyReduceInvertedBinaryPassArtifact(b).into_py_any(py)
        } else if let Ok(b) = ea.restore::<PyTransformationPassAdapterArtifact>() {
            b.into_py_any(py)
        } else {
            Err(LunaModelError::Internal(
                format!("failed to restore artifact for tag: {}", self.type_tag).into(),
            ))?
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum PyPassEntry {
    /// A transformation pass entry
    Transform {
        pass_id: String,
        pass_name: String,
        artifact: PyErasedArtifact,
    },

    /// An analysis pass (no artifact, not reversed)
    Analysis { pass_name: String },

    /// A meta-analysis pass (no artifact, not reversed)
    MetaAnalysis { pass_name: String },

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

    Composite {
        pass_id: String,
        pass_name: String,
        artifact: PyErasedArtifact,
    },
}

#[pymethods]
impl PyPassEntry {
    fn __str__(&self) -> String {
        match self {
            Self::Transform { pass_name, .. } => format!("TransformEntry({pass_name})"),
            Self::Composite { pass_name, .. } => format!("CompositeEntry({pass_name})"),
            Self::Analysis { pass_name } => format!("AnalysisEntry({pass_name})"),
            Self::MetaAnalysis { pass_name } => format!("MetaAnalysisEntry({pass_name})"),
            Self::Pipeline { name, .. } => format!("PipelineEntry({name})"),
            Self::ControlFlow { pass_name, .. } => format!("ControlFlowEntry({pass_name})"),
        }
    }
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
            PassEntry::Composite {
                pass_id,
                pass_name,
                artifact,
            } => Self::Composite {
                pass_id: pass_id.clone(),
                pass_name: pass_name.clone(),
                artifact: artifact.into(),
            },
            PassEntry::Analysis { pass_name } => Self::Analysis {
                pass_name: pass_name.clone(),
            },
            PassEntry::MetaAnalysis { pass_name } => Self::MetaAnalysis {
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
