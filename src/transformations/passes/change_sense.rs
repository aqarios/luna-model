use crate::{
    core::{operations::MulAssignToExpression, Model, Sense, Solution},
    transformations::{
        analysis_cache::AnalysisCache,
        base_passes::{
            ActionType, BasePass, TransformationOutcome, TransformationPass,
            TransformationPassResult,
        },
    },
};

#[cfg(feature = "py")]
use {crate::transformations::base_passes::Pass, aqm_macros::py_pass};

#[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
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
            model.objective.mul_assign(-1.0);
            model.set_sense(self.sense);
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
            .into_iter()
            .map(|x| x.map(|y| y * (-1.0)))
            .collect();
        solution
    }
}
