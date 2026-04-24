//! Unified pipeline-step enum for erased pass execution.

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

/// Type-erased step in a compiled pipeline.
#[derive(Clone)]
pub enum PipelineStep {
    /// A transformation pass.
    Transform(Arc<dyn ErasedTransformPass>),
    /// An analysis pass.
    Analysis(Arc<dyn ErasedAnalysisPass>),
    /// A meta-analysis pass.
    MetaAnalysis(Arc<dyn ErasedMetaAnalysisPass>),
    /// A control-flow pass.
    ControlFlow(Arc<dyn ErasedControlFlowPass>),
    /// A composite pass.
    Composite(Arc<dyn ErasedCompositePass>),
    /// A nested pipeline.
    Pipeline(Arc<Pipeline>),
}

impl PipelineStep {
    /// Returns a human-readable rendering of the step.
    pub fn display(&self) -> String {
        self.idisplay(0)
    }

    /// Returns an indented human-readable rendering of the step.
    pub fn idisplay(&self, indent: usize) -> String {
        match self {
            Self::Transform(p) => p.idisplay(indent),
            Self::Analysis(p) => p.idisplay(indent),
            Self::MetaAnalysis(p) => p.idisplay(indent),
            Self::ControlFlow(p) => p.idisplay(indent),
            Self::Composite(p) => p.idisplay(indent),
            Self::Pipeline(p) => p.idisplay(indent),
        }
    }
}

/// Trait for indentation-aware display helpers used by pipeline visualization.
pub trait IndentDisplay {
    /// Returns an indented string representation.
    fn idisplay(&self, indent: usize) -> String;
}

// TODO(team): fix these via implementation on common "Display" trait.

impl IndentDisplay for dyn ErasedTransformPass {
    fn idisplay(&self, indent: usize) -> String {
        let prefix = match indent {
            0 => String::default(),
            i => " ".repeat(i),
        };
        format!("{prefix}{}", self.display())
    }
}

impl IndentDisplay for dyn ErasedAnalysisPass {
    fn idisplay(&self, indent: usize) -> String {
        let prefix = match indent {
            0 => String::default(),
            i => " ".repeat(i),
        };
        format!("{prefix}{}", self.display())
    }
}

impl IndentDisplay for dyn ErasedMetaAnalysisPass {
    fn idisplay(&self, indent: usize) -> String {
        let prefix = match indent {
            0 => String::default(),
            i => " ".repeat(i),
        };
        format!("{prefix}{}", self.display())
    }
}

impl IndentDisplay for dyn ErasedControlFlowPass {
    fn idisplay(&self, indent: usize) -> String {
        let prefix = match indent {
            0 => String::default(),
            i => " ".repeat(i),
        };
        format!("{prefix}{}", self.display())
    }
}

impl IndentDisplay for dyn ErasedCompositePass {
    fn idisplay(&self, indent: usize) -> String {
        let prefix = match indent {
            0 => String::default(),
            i => " ".repeat(i),
        };
        format!("{prefix}{}", self.display())
    }
}

/// Formats a step slice with the given indentation.
fn _format_pipeline_steps(steps: &[PipelineStep], indent: usize) -> String {
    if steps.is_empty() {
        return String::default();
    }

    let mut items = Vec::new();
    if steps.len() >= 2 {
        for item in steps.iter().take((steps.len() - 1) + 1) {
            items.push(item.idisplay(indent).to_string());
        }
    }
    items.push(steps[steps.len() - 1].idisplay(indent).to_string());
    items.join("\n")
}

/// Display helpers for slices and vectors of [`PipelineStep`] values.
pub trait DisplaySteps {
    /// Returns a non-indented rendering.
    fn display(&self) -> String;
    /// Returns an indented rendering.
    fn idisplay(&self, indent: usize) -> String;
}

impl DisplaySteps for Vec<PipelineStep> {
    fn display(&self) -> String {
        _format_pipeline_steps(self, 0)
    }

    fn idisplay(&self, indent: usize) -> String {
        _format_pipeline_steps(self, indent)
    }
}

impl DisplaySteps for &[PipelineStep] {
    fn display(&self) -> String {
        _format_pipeline_steps(self, 0)
    }

    fn idisplay(&self, indent: usize) -> String {
        _format_pipeline_steps(self, indent)
    }
}
