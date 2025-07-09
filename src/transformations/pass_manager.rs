use super::execution::{backwards, run_passes};
use super::{
    analysis_cache::AnalysisCache, base_passes::Pass, errors::CompilationError,
    intermediate_representation::IntermediateRepresentation,
};
use crate::core::{Model, Solution};
use std::fmt;

#[derive(Debug)]
pub struct PassManager {
    pub passes: Vec<Pass>,
}

impl PassManager {
    pub fn new(passes: Option<Vec<Pass>>) -> PassManager {
        if let Some(x) = passes {
            PassManager { passes: x }
        } else {
            PassManager { passes: Vec::new() }
        }
    }

    pub fn add_pass(&mut self, pass: Pass) {
        self.passes.push(pass);
    }

    pub fn run(&self, model: Model) -> Result<IntermediateRepresentation, CompilationError> {
        run_passes(&self.passes, model, AnalysisCache::new())
    }

    pub fn backwards(&self, solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        backwards(&self.passes, solution, ir)
    }
}

impl fmt::Display for PassManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PassManager\n")?;
        for pass in self.passes.iter() {
            let s = match pass {
                Pass::Transformation(_) => "⚙️",
                Pass::Analysis(_) => "🔎",
                Pass::IfElse(_) => "❔",
                Pass::Pipeline(_) => "🛢️",
            };
            write!(f, "{} {}\n", s, pass.name())?;
        }
        Ok(())
    }
}
