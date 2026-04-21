use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{PassContext, Pipeline, PipelineStep};

#[derive(Clone)]
pub struct ControlFlowPlan {
    pub pipeline: Pipeline,
}

impl ControlFlowPlan {
    pub fn new(name: String, steps: Vec<PipelineStep>) -> Self {
        Self {
            pipeline: Pipeline { name, steps },
        }
    }

    pub fn name(&self) -> &str {
        &self.pipeline.name
    }

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

    /// Overridable to_string method for displaying the pass as human readble.
    fn display(&self) -> String {
        format!("🔀 {}", self.name())
    }
}
