use std::{collections::BTreeSet, sync::Arc};

use crate::{DisplaySteps, PipelineStep};

/// Named sequence of pipeline steps.
#[derive(Clone)]
pub struct Pipeline {
    /// Human-readable pipeline name.
    pub name: String,
    /// Ordered steps executed by the pipeline.
    pub steps: Vec<PipelineStep>,
}

impl From<Pipeline> for PipelineStep {
    /// Wraps a pipeline as a nested pipeline step.
    fn from(value: Pipeline) -> Self {
        PipelineStep::Pipeline(Arc::new(value))
    }
}

impl Pipeline {
    /// Creates a named pipeline from an explicit step list.
    pub fn new(name: String, steps: Vec<PipelineStep>) -> Self {
        Self { name, steps }
    }

    /// Returns the distinct requirement names referenced anywhere in the pipeline.
    pub fn requires(&self) -> impl Iterator<Item = String> {
        self.steps.collect_requires()
    }

    /// Returns the distinct analysis/provision names produced anywhere in the pipeline.
    pub fn provides(&self) -> impl Iterator<Item = String> {
        self.steps.collect_provides()
    }

    /// Returns the distinct analysis names invalidated anywhere in the pipeline.
    pub fn invalidates(&self) -> impl Iterator<Item = String> {
        self.steps.collect_invalidates()
    }

    /// Returns a human-readable rendering of the pipeline.
    pub fn display(&self) -> String {
        self.idisplay(0)
    }

    /// Returns an indented human-readable rendering of the pipeline.
    pub fn idisplay(&self, indent: usize) -> String {
        let prefix = match indent {
            0 => String::default(),
            i => " ".repeat(i),
        };
        format!(
            "{prefix}🛢️ {}  \n{}",
            self.name,
            self.steps.idisplay(indent + 2)
        )
    }

    /// Removes all configured steps from the pipeline.
    pub fn clear(&mut self) {
        self.steps = Vec::new();
    }
}

/// Introspection helpers over collections of [`PipelineStep`] values.
pub trait PipelineStepMethods {
    /// Collects distinct requirement names from the step tree.
    fn collect_requires(&self) -> impl Iterator<Item = String>;
    /// Collects distinct provided analysis names from the step tree.
    fn collect_provides(&self) -> impl Iterator<Item = String>;
    /// Collects distinct invalidated analysis names from the step tree.
    fn collect_invalidates(&self) -> impl Iterator<Item = String>;
}

impl PipelineStepMethods for [PipelineStep] {
    /// Walks the step tree and collects all requirement names.
    fn collect_requires(&self) -> impl Iterator<Item = String> {
        let mut out = BTreeSet::<String>::new();
        fn walk(steps: &[PipelineStep], out: &mut BTreeSet<String>) {
            for step in steps {
                match step {
                    PipelineStep::Transform(p) => out.extend(p.requires().to_owned()),
                    PipelineStep::Analysis(p) => out.extend(p.requires().to_owned()),
                    PipelineStep::ControlFlow(p) => out.extend(p.requires().to_owned()),
                    PipelineStep::Composite(p) => out.extend(p.requires().to_owned()),
                    PipelineStep::Pipeline(p) => walk(&p.steps, out),
                    PipelineStep::MetaAnalysis(_) => (),
                }
            }
        }
        walk(self, &mut out);
        out.into_iter()
    }

    /// Walks the step tree and collects all provided analysis names.
    fn collect_provides(&self) -> impl Iterator<Item = String> {
        let mut out = BTreeSet::<String>::new();
        fn walk(steps: &[PipelineStep], out: &mut BTreeSet<String>) {
            for step in steps {
                match step {
                    PipelineStep::Transform(_) => (),
                    PipelineStep::Analysis(p) => _ = out.insert(p.provides().to_owned()),
                    PipelineStep::MetaAnalysis(p) => _ = out.insert(p.provides().to_owned()),
                    PipelineStep::ControlFlow(p) => out.extend(p.provides().to_owned()),
                    PipelineStep::Composite(p) => _ = out.insert(p.provides().to_owned()),
                    PipelineStep::Pipeline(p) => walk(&p.steps, out),
                }
            }
        }
        walk(self, &mut out);
        out.into_iter()
    }

    /// Walks the step tree and collects all invalidated analysis names.
    fn collect_invalidates(&self) -> impl Iterator<Item = String> {
        let mut out = BTreeSet::<String>::new();
        fn walk(steps: &[PipelineStep], out: &mut BTreeSet<String>) {
            for step in steps {
                match step {
                    PipelineStep::Analysis(_) | PipelineStep::MetaAnalysis(_) => (),
                    PipelineStep::Transform(p) => out.extend(p.invalidates().to_owned()),
                    PipelineStep::ControlFlow(p) => out.extend(p.invalidates().to_owned()),
                    PipelineStep::Composite(p) => out.extend(p.invalidates().to_owned()),
                    PipelineStep::Pipeline(p) => walk(&p.steps, out),
                }
            }
        }
        walk(self, &mut out);
        out.into_iter()
    }
}
