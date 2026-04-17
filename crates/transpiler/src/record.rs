use lunamodel_core::Solution;
use lunamodel_error::{LunaModelError, LunaModelResult};

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

    /// A meta analysis pass (no artifact, not reversed)
    MetaAnalysis { pass_name: String },

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

    /// A composite pass with its artifact
    Composite {
        pass_id: String,
        pass_name: String,
        artifact: ErasedArtifact,
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
                }
                | PassEntry::Composite {
                    pass_id, artifact, ..
                } => {
                    // Look up the backwards function and apply it
                    registry::apply(pass_id, artifact, solution)?
                }

                PassEntry::Pipeline { record, .. } | PassEntry::ControlFlow { record, .. } => {
                    // Recursively apply backwards through sub-pipeline
                    record.backward(solution)?
                }

                PassEntry::Analysis { .. } | PassEntry::MetaAnalysis { .. } => {
                    // Analysis & MetaAnalysis passes don't affect backwards
                    solution
                }
            };
        }

        Ok(solution)
    }

    pub fn find(&self, query: &str, exact: bool) -> LunaModelResult<&PassEntry> {
        if query.is_empty() {
            return Err(LunaModelError::Computation("query must not be empty".into()).into());
        }

        if exact {
            self.find_exact(query).ok_or_else(|| {
                LunaModelError::Computation(
                    format!(
                        "no exact entry match found for query 
'{query}'"
                    )
                    .into(),
                )
                .into()
            })
        } else {
            let needle = query.to_lowercase();
            self.find_partial(&needle).ok_or_else(|| {
                LunaModelError::Computation(
                    format!("no partial entry match found for query '{query}'").into(),
                )
                .into()
            })
        }
    }

    fn find_exact(&self, query: &str) -> Option<&PassEntry> {
        for entry in &self.entries {
            if entry_matches_exact(entry, query) {
                return Some(entry);
            }

            match entry {
                PassEntry::Pipeline { record, .. } | PassEntry::ControlFlow { record, .. } => {
                    if let Some(hit) = record.find_exact(query) {
                        return Some(hit);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn find_partial(&self, needle_lower: &str) -> Option<&PassEntry> {
        for entry in &self.entries {
            if entry_matches_partial(entry, needle_lower) {
                return Some(entry);
            }

            match entry {
                PassEntry::Pipeline { record, .. } | PassEntry::ControlFlow { record, .. } => {
                    if let Some(hit) = record.find_partial(needle_lower) {
                        return Some(hit);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl From<Vec<PassEntry>> for TransformationRecord {
    fn from(entries: Vec<PassEntry>) -> Self {
        Self { entries }
    }
}

fn entry_matches_exact(entry: &PassEntry, query: &str) -> bool {
    match entry {
        PassEntry::Transform {
            pass_id, pass_name, ..
        }
        | PassEntry::Composite {
            pass_id, pass_name, ..
        } => pass_name == query || pass_id == query,
        PassEntry::Analysis { pass_name } | PassEntry::MetaAnalysis { pass_name } => {
            pass_name == query
        }
        PassEntry::Pipeline { name, .. } => name == query,
        PassEntry::ControlFlow {
            pass_name, name, ..
        } => pass_name == query || name == query,
    }
}

fn entry_matches_partial(entry: &PassEntry, needle_lower: &str) -> bool {
    match entry {
        PassEntry::Transform {
            pass_id, pass_name, ..
        }
        | PassEntry::Composite {
            pass_id, pass_name, ..
        } => {
            pass_name.to_lowercase().contains(needle_lower)
                || pass_id.to_lowercase().contains(needle_lower)
        }
        PassEntry::Analysis { pass_name } | PassEntry::MetaAnalysis { pass_name } => {
            pass_name.to_lowercase().contains(needle_lower)
        }
        PassEntry::Pipeline { name, .. } => name.to_lowercase().contains(needle_lower),
        PassEntry::ControlFlow {
            pass_name, name, ..
        } => {
            pass_name.to_lowercase().contains(needle_lower)
                || name.to_lowercase().contains(needle_lower)
        }
    }
}
