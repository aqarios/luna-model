use lunamodel_transpiler::PipelineStep;
use pyo3::pyclass;

/// Python-facing typed view of pipeline steps for meta-analysis passes.
///
/// Each variant carries only stable metadata required by Python-side
/// `MetaAnalysisPass` implementations.
#[pyclass]
#[derive(Clone)]
pub enum PyStepView {
    /// View of a transformation step.
    Transform {
        name: String,
        requires: Vec<String>,
        invalidates: Vec<String>,
    },

    /// View of an analysis step.
    Analysis {
        name: String,
        provides: String,
        requires: Vec<String>,
    },

    /// View of a meta-analysis step.
    MetaAnalysis {
        name: String,
        provides: String,
    },

    /// View of a control-flow step.
    ControlFlow {
        name: String,
        requires: Vec<String>,
        provides: Vec<String>,
        invalidates: Vec<String>,
    },

    /// View of a composite step.
    Composite {
        name: String,
        requires: Vec<String>,
        provides: String,
        invalidates: Vec<String>,
    },

    /// View of a nested pipeline with recursively nested step views.
    Pipeline {
        name: String,
        requires: Vec<String>,
        provides: Vec<String>,
        invalidates: Vec<String>,
        nested: Vec<PyStepView>,
    },
}

impl From<&PipelineStep> for PyStepView {
    /// Convert a transpiler pipeline step into its Python-facing view.
    fn from(value: &PipelineStep) -> Self {
        match value {
            PipelineStep::Transform(p) => PyStepView::Transform {
                name: p.name().to_string(),
                requires: p.requires().to_vec(),
                invalidates: p.invalidates().to_vec(),
            },
            PipelineStep::Analysis(p) => PyStepView::Analysis {
                name: p.name().to_string(),
                provides: p.provides().to_string(),
                requires: p.requires().to_vec(),
            },
            PipelineStep::MetaAnalysis(p) => PyStepView::MetaAnalysis {
                name: p.name().to_string(),
                provides: p.provides().to_string(),
            },
            PipelineStep::ControlFlow(p) => PyStepView::ControlFlow {
                name: p.name().to_string(),
                requires: p.requires().to_vec(),
                provides: p.provides().to_vec(),
                invalidates: p.invalidates().to_vec(),
            },
            PipelineStep::Composite(p) => PyStepView::Composite {
                name: p.name().to_string(),
                requires: p.requires().to_vec(),
                provides: p.provides().to_string(),
                invalidates: p.invalidates().to_vec(),
            },
            PipelineStep::Pipeline(p) => PyStepView::Pipeline {
                name: p.name.to_string(),
                requires: p.requires().collect(),
                provides: p.provides().collect(),
                invalidates: p.invalidates().collect(),
                nested: p.steps.iter().map(|s| s.into()).collect(),
            },
        }
    }
}

pub trait FromSteps {
    /// Convert a slice of pipeline steps into Python-facing views.
    fn to_views(&self) -> Vec<PyStepView>;
}

impl FromSteps for &[PipelineStep] {
    /// Convert all steps in this slice into Python-facing views.
    fn to_views(&self) -> Vec<PyStepView> {
        self.iter().map(|s| s.into()).collect()
    }
}
