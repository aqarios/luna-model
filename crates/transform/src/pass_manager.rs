use std::fmt::{Display, Formatter, Result};

use lunamodel_core::prelude::{Model, Solution};
use lunamodel_error::LunaModelResult;

use crate::{
    base::Pass,
    cache::AnalysisCache,
    execution::{backwards, check_dependencies, run_passes},
    ir::IR,
};

#[derive(Debug)]
pub struct PassManager {
    passes: Vec<Pass>,
}

impl PassManager {
    pub fn new(passes: Option<Vec<Pass>>) -> PassManager {
        PassManager {
            passes: passes.unwrap_or_else(|| Vec::default()),
        }
    }

    pub fn add_pass(&mut self, pass: Pass) {
        self.passes.push(pass);
    }

    pub fn passes(&self) -> &[Pass] {
        &self.passes
    }

    pub fn run(&self, model: Model) -> LunaModelResult<IR> {
        let input_model = model.deep_clone();
        check_dependencies(&self.passes)?;
        let mut ir = run_passes(&self.passes, model, AnalysisCache::new(), self)?;
        ir.input_model = Some(input_model);
        return Ok(ir);
    }

    pub fn backwards(&self, solution: Solution, ir: &IR) -> Solution {
        // TODO: needs Backwards Error
        let mut sol = backwards(&self.passes, solution, ir, None);
        // NOTE: might want to add this again...
        // if let Some(x) = &sol.obj_values {
        //     if sol.n_samples() > 0 && sol.raw_energies.is_none() {
        //         sol.raw_energies = x.iter().map(|&y| Some(y)).collect();
        //     }
        // }
        sol.raw_energies = None;
        if let Some(input) = &ir.input_model {
            input.evaluate_solution(&sol).unwrap()
        } else {
            sol
        }
    }
}

impl Display for PassManager {
    // TODO: move Dispaly to lunamodel_io
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
