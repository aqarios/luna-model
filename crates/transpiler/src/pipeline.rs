use std::{collections::BTreeSet, sync::Arc};

use crate::{DisplaySteps, PipelineStep};

#[derive(Clone)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<PipelineStep>,
}

impl From<Pipeline> for PipelineStep {
    fn from(value: Pipeline) -> Self {
        PipelineStep::Pipeline(Arc::new(value))
    }
}

impl Pipeline {
    pub fn new(name: String, steps: Vec<PipelineStep>) -> Self {
        Self { name, steps }
    }

    pub fn requires(&self) -> impl Iterator<Item = String> {
        self.steps.collect_requires()
    }

    pub fn provides(&self) -> impl Iterator<Item = String> {
        self.steps.collect_provides()
    }

    pub fn invalidates(&self) -> impl Iterator<Item = String> {
        self.steps.collect_invalidates()
    }

    pub fn display(&self) -> String {
        self.idisplay(0)
    }

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

    pub fn clear(&mut self) {
        self.steps = Vec::new();
    }
}

pub trait PipelineStepMethods {
    fn collect_requires(&self) -> impl Iterator<Item = String>;
    fn collect_provides(&self) -> impl Iterator<Item = String>;
    fn collect_invalidates(&self) -> impl Iterator<Item = String>;
}

impl PipelineStepMethods for [PipelineStep] {
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
