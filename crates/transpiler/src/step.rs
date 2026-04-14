use std::sync::Arc;

use crate::{
    Pipeline,
    erased::{
        ErasedAnalysisPass, ErasedCompositePass, ErasedControlFlowPass, ErasedMetaAnalysisPass,
        ErasedTransformPass,
    },
};

// Note: PipelineStep is intentionally Arc-backed so `from_steps(steps.clone())`
// is cheap and does not require cloning non-cloneable closures or trait objects.

#[derive(Clone)]
pub enum PipelineStep {
    Transform(Arc<dyn ErasedTransformPass>),
    Analysis(Arc<dyn ErasedAnalysisPass>),
    MetaAnalysis(Arc<dyn ErasedMetaAnalysisPass>),
    ControlFlow(Arc<dyn ErasedControlFlowPass>),
    Composite(Arc<dyn ErasedCompositePass>),
    Pipeline(Arc<Pipeline>),
}

impl PipelineStep {
    pub fn display(&self) -> String {
        match self {
            Self::Transform(p) => p.display(),
            Self::Analysis(p) => p.display(),
            Self::MetaAnalysis(p) => p.display(),
            Self::ControlFlow(p) => p.display(),
            Self::Composite(p) => p.display(),
            Self::Pipeline(p) => p.display(),
        }
    }
}

fn _format_pipeline_steps(steps: &[PipelineStep]) -> String {
    if steps.is_empty() {
        return String::default();
    }

    let mut items = Vec::new();
    if steps.len() >= 2 {
        for i in 0..=steps.len() - 2 {
            items.push(steps[i].display())
        }
    }
    items.push(steps[steps.len() - 1].display());
    items.join("\n")
}

pub trait DisplaySteps {
    fn display(&self) -> String;
}

impl DisplaySteps for Vec<PipelineStep> {
    fn display(&self) -> String {
        _format_pipeline_steps(&self)
    }
}

impl DisplaySteps for &[PipelineStep] {
    fn display(&self) -> String {
        _format_pipeline_steps(&self)
    }
}
