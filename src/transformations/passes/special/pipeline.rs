use crate::core::{Model, Solution};
use crate::transformations::base_passes::TransformationPass;
use crate::{
    transformations::analysis_cache::AnalysisCache,
    transformations::base_passes::{BasePass, Pass},
    transformations::errors::CompilationError,
    transformations::execution::{backwards, run_passes},
    transformations::intermediate_representation::IntermediateRepresentation,
};
use global_counter::primitive::exact::CounterU64;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::fmt::Display;

/// Collection of Passes that are executed in the order the pipeline is initialized.
#[derive(Debug, Clone)]
pub struct Pipeline {
    passes: Vec<Pass>,
    required: HashSet<String>,
    satisfied: HashSet<String>,
    name: String,
}

/// Counter to ensure multiple if-else branches can be used in the same pass.
pub static PIPELINE_COUNTER: CounterU64 = CounterU64::new(0);

impl Pipeline {
    pub fn new(passes: Vec<Pass>, name: Option<String>) -> Self {
        let mut required = HashSet::new();
        let mut satisfied = HashSet::new();
        for pass in passes.iter() {
            for req in pass.requires().iter() {
                if !satisfied.contains(req) {
                    required.insert(req.clone());
                }
            }
            if let Pass::Transformation(x) = pass {
                for inv in x.invalidates().iter() {
                    satisfied.remove(inv);
                }
            }
            if let Pass::Pipeline(pipeline) = pass {
                satisfied.extend(pipeline.satisfied())
            }
            satisfied.insert(pass.name());
        }
        Self {
            required,
            passes,
            satisfied,
            name: name.unwrap_or(format!("pipeline-{}", PIPELINE_COUNTER.inc())),
        }
    }

    pub fn satisfied(&self) -> HashSet<String> {
        self.satisfied.clone()
    }
}

impl BasePass for Pipeline {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn requires(&self) -> Vec<String> {
        self.required.iter().cloned().collect()
    }


}

pub type PipelineResult = Result<IntermediateRepresentation, CompilationError>;

impl Pipeline {
    pub fn run(&self, model: Model, cache: &AnalysisCache) -> PipelineResult {
        run_passes(&self.passes, model, cache.clone())
    }

    pub fn backwards(&self, solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        backwards(&self.passes, solution, ir)
    }

    pub fn clear(&mut self) {
        self.passes.clear()
    }

    pub fn add(&mut self, pass: Pass) {
        for req in pass.requires().iter() {
            if !self.satisfied.contains(req) {
                self.required.insert(req.clone());
            }
            if let Pass::Transformation(x) = &pass {
                for inv in x.invalidates().iter() {
                    self.satisfied.remove(inv);
                }
            }
            if let Pass::Pipeline(pipeline) = &pass {
                self.satisfied.extend(pipeline.satisfied())
            }
            self.satisfied.insert(pass.name());
        }
        self.passes.push(pass)
    }
}

impl Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🛢️ {}\n", self.name)?;
        write!(f, "{}", self.content_string().replace("\n", "\n  "))?;
        Ok(())
    }
}

impl Pipeline {
    pub fn content_string(&self) -> String {
        let mut out = String::new();
        if self.passes.len() >= 2 {
            for i in 0..=self.passes.len() - 2 {
                out += &format!("{}\n", self.passes[i]);
            }
        }
        if self.passes.len() >= 1 {
            out += &format!("{}", self.passes[self.passes.len() - 1]);
        }
        out
    }

    pub fn len(&self) -> usize {
        self.passes.len()
    }
}
