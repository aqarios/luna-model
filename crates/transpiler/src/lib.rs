mod analysis;
mod artifact;
mod context;
mod control_flow;
mod erased;
mod error;
mod output;
mod manager;
mod pass;
mod pipeline;
mod record;
mod registry;

pub use analysis::{AnalysisKey, AnalysisManager};
pub use artifact::{Artifact, ErasedArtifact};
pub use context::PassContext;
pub use control_flow::{ControlFlowPass, ControlFlowPlan};
pub use erased::{ErasedAnalysisPass, ErasedTransformPass};
pub use error::TransformationError;
pub use output::TransformationOutput;
pub use manager::{PassManager, PipelineStep};
pub use pass::{AnalysisPass, ReversiblePass};
pub use pipeline::{Pipeline, PipelineStepMethods};
pub use record::{TransformationRecord, PassEntry};
pub use registry::{BackwardRegistry, apply as apply_backward, register_backward};

pub mod prelude {
    pub use crate::{
        AnalysisKey, AnalysisManager, AnalysisPass, Artifact, BackwardRegistry, TransformationRecord,
        ControlFlowPass, ControlFlowPlan, ErasedAnalysisPass, ErasedArtifact, ErasedTransformPass,
        TransformationOutput, PassContext, PassEntry, PassManager, Pipeline, PipelineStep, PipelineStepMethods,
        ReversiblePass, TransformationError, apply_backward, register_backward,
    };
}
