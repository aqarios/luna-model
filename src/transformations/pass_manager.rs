use super::execution::{backwards, check_dependencies, run_passes};
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
        let input_model = model.deep_clone();
        check_dependencies(&self.passes)?;
        let mut ir = run_passes(&self.passes, model, AnalysisCache::new(), self)?;
        ir.input_model = Some(input_model);
        return Ok(ir)
    }

    pub fn backwards(&self, solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        // TODO: needs Backwards Error
        let mut sol = backwards(&self.passes, solution, ir, None);
        if let Some(x) = &sol.obj_values {
            if sol.n_samples > 0 && sol.raw_energies.is_none() {
                sol.raw_energies = x.iter().map(|&y| Some(y)).collect();
            }
        }
        if let Some(input) = &ir.input_model {
            input.evaluate_solution(sol).unwrap()
        } else {
            sol
        }
    }
}

impl fmt::Display for PassManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PassManager\n")?;
        if self.passes.len() >= 2 {
            for i in 0..=self.passes.len() - 2 {
                write!(f, "{}\n", self.passes[i])?;
            }
        }
        write!(f, "{}", self.passes[self.passes.len() - 1])?;
        Ok(())
    }
}
