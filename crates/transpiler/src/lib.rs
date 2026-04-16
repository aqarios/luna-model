mod analysis;
mod artifact;
mod reversible;
mod composite;
mod context;
mod control_flow;
mod erased;
mod error;
mod manager;
mod meta;
mod output;
mod pass;
mod pipeline;
mod record;
mod registry;
mod step;

pub use analysis::{AnalysisKey, AnalysisManager};
pub use artifact::{Artifact, ErasedArtifact};
pub use reversible::Reversible;
pub use composite::CompositePass;
pub use context::PassContext;
pub use control_flow::{ControlFlowPass, ControlFlowPlan};
pub use error::TransformationError;
pub use manager::PassManager;
pub use meta::MetaAnalysisPass;
pub use output::TransformationOutput;
pub use pass::{AnalysisPass, TransformationPass};
pub use pipeline::{Pipeline, PipelineStepMethods};
pub use record::{PassEntry, TransformationRecord};
pub use registry::{BackwardRegistry, apply as apply_backward, register_backward};
pub use step::{DisplaySteps, PipelineStep};

pub use lunamodel_transpiler_macros::{analysis, control_flow, transformation};

pub mod prelude {
    pub use crate::{
        AnalysisKey, AnalysisManager, AnalysisPass, Artifact, BackwardRegistry, Reversible,
        CompositePass, ControlFlowPass, ControlFlowPlan, ErasedArtifact, MetaAnalysisPass,
        PassContext, PassEntry, PassManager, Pipeline, PipelineStep, PipelineStepMethods,
        TransformationPass, TransformationError, TransformationOutput, TransformationRecord,
        apply_backward, register_backward,
    };
    pub use lunamodel_transpiler_macros::{analysis, composite, control_flow, transformation};
}
