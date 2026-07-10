//! Generic pass orchestration framework used by LunaModel transformations.
//!
//! The `transpiler` crate is the execution engine behind analyses,
//! transformations, composite passes, control-flow passes, pipelines, and
//! reversible transformation records. It is intentionally generic so the core
//! transformation logic can live in domain-specific crates while still sharing a
//! single execution model.
mod analysis;
mod artifact;
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
mod reversible;
mod step;
mod validation;

pub use analysis::{AnalysisKey, AnalysisManager};
pub use artifact::{Artifact, ErasedArtifact};
pub use composite::CompositePass;
pub use context::PassContext;
pub use control_flow::{ControlFlowPass, ControlFlowPlan};
pub use error::{TranspileErrorKind, TranspileKindResult, TranspilerError, TranspilerResult};
pub use manager::PassManager;
pub use meta::MetaAnalysisPass;
pub use output::TransformationOutput;
pub use pass::{AnalysisPass, TransformationPass};
pub use pipeline::{Pipeline, PipelineStepMethods};
pub use record::{PassEntry, TransformationRecord};
pub use registry::{BackwardRegistry, apply as apply_backward, register_backward};
pub use reversible::Reversible;
pub use step::{DisplaySteps, PipelineStep};

/// Attribute macros used by downstream pass implementations.
pub use lunamodel_transpiler_macros::{analysis, control_flow, transformation};

/// Convenience re-export module for downstream crates defining passes and pipelines.
pub mod prelude {
    pub use crate::{
        AnalysisKey, AnalysisManager, AnalysisPass, Artifact, BackwardRegistry, CompositePass,
        ControlFlowPass, ControlFlowPlan, ErasedArtifact, MetaAnalysisPass, PassContext, PassEntry,
        PassManager, Pipeline, PipelineStep, PipelineStepMethods, Reversible, TransformationOutput,
        TransformationPass, TransformationRecord, TranspilerError, apply_backward,
        register_backward,
    };
    pub use lunamodel_transpiler_macros::{analysis, composite, control_flow, transformation};
}
