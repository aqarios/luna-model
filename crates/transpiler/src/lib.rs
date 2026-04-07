mod analysis;
mod artifact;
mod context;
mod error;
mod manager;
mod pass;
mod record;
mod registry;
mod pipeline;

pub use analysis::{AnalysisKey, AnalysisManager};
pub use artifact::{Artifact, ErasedArtifact};
pub use context::PassContext;
pub use error::TransformationError;
pub use manager::{ErasedAnalysisPass, ErasedTransformPass, PassManager, PipelineStep};
pub use pass::{AnalysisPass, ReversiblePass};
pub use record::{CompilationRecord, PassEntry};
pub use registry::{BackwardRegistry, apply as apply_backward, register_backward};
pub use pipeline::{Pipeline, PipelineStepRequires};

pub mod prelude {
    pub use crate::{
        AnalysisKey, AnalysisManager, AnalysisPass, Artifact, BackwardRegistry, CompilationRecord,
        ErasedAnalysisPass, ErasedArtifact, ErasedTransformPass, PassContext, PassEntry,
        PassManager, PipelineStep, ReversiblePass, TransformationError, apply_backward,
        register_backward, Pipeline, PipelineStepRequires
    };
}
