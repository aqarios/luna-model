use std::sync::Arc;

use crate::erased::{ErasedAnalysisPass, ErasedControlFlowPass, ErasedTransformPass};

// Note: PipelineStep is intentionally Arc-backed so `from_steps(steps.clone())`
// is cheap and does not require cloning non-cloneable closures or trait objects.

#[derive(Clone)]
pub enum PipelineStep {
    Transform(Arc<dyn ErasedTransformPass>),
    Analysis(Arc<dyn ErasedAnalysisPass>),
    ControlFlow(Arc<dyn ErasedControlFlowPass>),
    Pipeline {
        name: String,
        passes: Vec<PipelineStep>,
    },
}
