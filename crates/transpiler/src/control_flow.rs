use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use crate::{PassContext, Pipeline, PipelineStep};

pub struct ControlFlowPlan {
    pub pipeline: Pipeline,
}

impl ControlFlowPlan {
    pub fn new(name: String, steps: Vec<PipelineStep>) -> Self {
        Self {
            pipeline: Pipeline { name, steps },
        }
    }
}

/// A control flow pass.
///
/// The forward pass generates a Pipeline that transforms or analyses a model and produces an artifact.
pub trait ControlFlowPass: Send + Sync {
    /// Name for this pass.
    fn name(&self) -> &str;

    /// Forward transformation: Model -> TransformedModel + Artifact
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
}
