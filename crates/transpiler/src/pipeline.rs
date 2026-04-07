use std::collections::BTreeSet;

use crate::PipelineStep;

#[derive(Clone)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<PipelineStep>,
}

impl From<Pipeline> for PipelineStep {
    fn from(value: Pipeline) -> Self {
        PipelineStep::Pipeline {
            name: value.name,
            passes: value.steps,
        }
    }
}

pub trait PipelineStepRequires {
    fn collect_requires(&self) -> impl Iterator<Item = String>;
}

impl PipelineStepRequires for [PipelineStep] {
    fn collect_requires(&self) -> impl Iterator<Item = String> {
        let mut out = BTreeSet::<String>::new();
        fn walk(steps: &[PipelineStep], out: &mut BTreeSet<String>) {
            for step in steps {
                match step {
                    PipelineStep::Transform(p) => out.extend(p.requires().to_owned()),
                    PipelineStep::Analysis(p) => out.extend(p.requires().to_owned()),
                    PipelineStep::Pipeline { passes, .. } => walk(passes, out),
                }
            }
        }
        walk(self, &mut out);
        out.into_iter()
    }
}
