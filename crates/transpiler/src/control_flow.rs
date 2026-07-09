//! Trait definitions for control-flow passes and execution plans.

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{PassContext, Pipeline, PipelineStep};

/// Concrete execution plan produced by a control-flow pass.
#[derive(Clone)]
pub struct ControlFlowPlan {
    /// Pipeline selected by the control-flow decision.
    pub pipeline: Pipeline,
}

impl ControlFlowPlan {
    /// Creates a plan from a pipeline name and step list.
    pub fn new(name: String, steps: Vec<PipelineStep>) -> Self {
        Self {
            pipeline: Pipeline { name, steps },
        }
    }

    /// Returns the selected pipeline name.
    pub fn name(&self) -> &str {
        &self.pipeline.name
    }

    /// Returns the selected pipeline steps.
    pub fn steps(&self) -> &[PipelineStep] {
        &self.pipeline.steps
    }
}

/// A control flow pass.
///
/// The forward pass generates a Pipeline that transforms or analyses a model and produces an artifact.
pub trait ControlFlowPass: Send + Sync {
    /// Name for this pass.
    fn name(&self) -> &str;

    /// Run control flow: Model + PassContext -> ControlFlowPlan
    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan>;

    /// Which pass/analysis keys must be satisfied before this pass can execute?
    fn requires(&self) -> &[String] {
        &[]
    }

    /// Which pass/analysis keys are provided by this pass.
    fn provides(&self) -> &[String] {
        &[]
    }

    /// Which pass/analysis keys might be invalidated after this pass was executed.
    fn invalidates(&self) -> &[String] {
        &[]
    }

    /// All possible branches (sub-steps) this pass may branch into, for static validation.
    fn branches(&self) -> Vec<&[PipelineStep]> {
        Vec::new()
    }

    /// Human-readable display string used by pipeline visualization.
    fn display(&self) -> String {
        format!("🔀 {}", self.name())
    }
}
