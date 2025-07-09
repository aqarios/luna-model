use std::fmt::{Display, Write};

use global_counter::primitive::exact::CounterU64;

use crate::core::{Model, Solution};

use crate::transformations::passes;
use crate::{
    transformations::analysis_cache::AnalysisCache,
    transformations::base_passes::{BasePass, Pass},
    transformations::errors::CompilationError,
    transformations::execution::{backwards, run_passes},
    transformations::intermediate_representation::IntermediateRepresentation,
};

/// Collection of Passes that are executed in the order the pipeline is initialized.
#[derive(Debug, Clone)]
pub struct Pipeline {
    passes: Vec<Pass>,
    name: String,
}

/// Counter to ensure multiple if-else branches can be used in the same pass.
pub static PIPELINE_COUNTER: CounterU64 = CounterU64::new(0);

impl Pipeline {
    pub fn new(passes: Vec<Pass>, name: Option<String>) -> Self {
        Self {
            passes,
            name: name.unwrap_or(format!("pipeline-{}", PIPELINE_COUNTER.inc())),
        }
    }
}

impl BasePass for Pipeline {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn requires(&self) -> Vec<String> {
        Vec::new()
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
