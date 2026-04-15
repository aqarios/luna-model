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
        self.idisplay(0)
    }

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

pub trait IndentDisplay {
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

fn _format_pipeline_steps(steps: &[PipelineStep], indent: usize) -> String {
    if steps.is_empty() {
        return String::default();
    }

    let mut items = Vec::new();
    if steps.len() >= 2 {
        for i in 0..=steps.len() - 2 {
            items.push(format!("{}", steps[i].idisplay(indent)))
        }
    }
    items.push(format!("{}", steps[steps.len() - 1].idisplay(indent)));
    items.join("\n")
}

pub trait DisplaySteps {
    fn display(&self) -> String;
    fn idisplay(&self, indent: usize) -> String;
}

impl DisplaySteps for Vec<PipelineStep> {
    fn display(&self) -> String {
        _format_pipeline_steps(&self, 0)
    }

    fn idisplay(&self, indent: usize) -> String {
        _format_pipeline_steps(&self, indent)
    }
}

impl DisplaySteps for &[PipelineStep] {
    fn display(&self) -> String {
        _format_pipeline_steps(&self, 0)
    }

    fn idisplay(&self, indent: usize) -> String {
        _format_pipeline_steps(&self, indent)
    }
}
