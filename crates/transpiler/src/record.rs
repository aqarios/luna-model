use lunamodel_core::Solution;
use lunamodel_error::LunaModelResult;

use crate::{artifact::ErasedArtifact, registry};

/// A record of the forward compilation, structured for backwards execution.
///
/// This is the serializable artifact of a full pass pipeline run.
#[derive(Debug, Clone)]
pub struct TransformationRecord {
    /// The sequence of transformations applied, in forward order
    pub(crate) entries: Vec<PassEntry>,
}

/// A single entry in the compilation record
#[derive(Debug, Clone)]
pub enum PassEntry {
    /// A transformation pass with its artifact
    Transform {
        pass_id: String,
        pass_name: String,
        artifact: ErasedArtifact,
    },

    /// An analysis pass (no artifact, not reversed)
    Analysis { pass_name: String },

    /// A nested sub-pipeline
    Pipeline {
        name: String,
        record: TransformationRecord,
    },

    /// A nested sub-pipeline originating from a ControlFlow
    ControlFlow {
        pass_name: String,
        name: String,
        record: TransformationRecord,
    },
}

impl TransformationRecord {
    pub fn entries(&self) -> impl Iterator<Item = &PassEntry> {
        self.entries.iter()
    }

    /// Execute backwards transformation
    ///
    /// This is a standalone function that doesn't need the original PassManager.
    /// All information is encoded in the artifacts.
    pub fn backward(&self, mut solution: Solution) -> LunaModelResult<Solution> {
        // Reverse order: last transformation first
        for entry in self.entries.iter().rev() {
            solution = match entry {
                PassEntry::Transform {
                    pass_id, artifact, ..
                } => {
                    // Look up the backwards function and apply it
                    registry::apply(pass_id, artifact, solution)?
                }

                PassEntry::Analysis { .. } => {
                    // Analysis passes don't affect backwards
                    solution
                }

                PassEntry::Pipeline { record, .. } => {
                    // Recursively apply backwards through sub-pipeline
                    record.backward(solution)?
                }
                PassEntry::ControlFlow { record, .. } => {
                    // Recursively apply backwards through sub-pipeline
                    record.backward(solution)?
                }
            };
        }

        Ok(solution)
    }
}

impl From<Vec<PassEntry>> for TransformationRecord {
    fn from(entries: Vec<PassEntry>) -> Self {
        Self { entries }
    }
}
