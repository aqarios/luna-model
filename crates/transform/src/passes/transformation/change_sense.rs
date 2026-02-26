use lunamodel_core::{Model, Solution, ops::LmMulAssign};
use lunamodel_types::Sense;

use crate::{
    base::{
        ActionType, BasePass, TransformationOutcome, TransformationPass, TransformationPassResult,
    },
    cache::AnalysisCache,
};

// #[cfg(feature = "py")]
// use {crate::base::Pass, lunamodel_tpass::py_pass, lunamodel_unwind::*};
//
// #[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
#[derive(Debug, Clone)]
pub struct ChangeSensePass {
    pub sense: Sense,
}

impl ChangeSensePass {
    pub fn new(sense: Sense) -> Self {
        ChangeSensePass { sense }
    }
}

impl BasePass for ChangeSensePass {
    fn name(&self) -> String {
        String::from("change-sense")
    }
}

impl TransformationPass for ChangeSensePass {
    fn run(&self, mut model: Model, _cache: &AnalysisCache) -> TransformationPassResult {
        if model.sense == self.sense {
            return Ok(TransformationOutcome::new(
                model,
                None,
                ActionType::DidNothing,
            ));
        } else {
            model.objective.mul_assign(-1.0)?;
            model.sense = self.sense;
            return Ok(TransformationOutcome::new(
                model,
                None,
                ActionType::DidTransform,
            ));
        }
    }

    fn backwards(&self, mut solution: Solution, _cache: &AnalysisCache) -> Solution {
        solution.obj_values = solution
            .obj_values
            .map(|x| x.into_iter().map(|y| y * (-1.0)).collect::<Vec<_>>());
        solution
    }

    fn invalidates(&self) -> Vec<String> {
        vec![String::from("specs")]
    }
}
