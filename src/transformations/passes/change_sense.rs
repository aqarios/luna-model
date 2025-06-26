use crate::{
    core::{operations::MulAssignToExpression, Model, Sense, Solution},
    transformations::{
        analysis_cache::AnalysisCache,
        base_passes::{BasePass, TransformationPass, TransformationPassResult, TransformationType},
    },
};

#[cfg(feature = "py")]
use crate::transformations::base_passes::Pass;
#[cfg(feature = "py")]
use aqm_macros::py_pass;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
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

    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
}

impl TransformationPass for ChangeSensePass {
    fn invalidates(&self) -> &[&str] {
        &[]
    }

    fn run(&self, mut model: Model, _cache: &AnalysisCache) -> TransformationPassResult {
        if model.sense == self.sense {
            return Ok((model, TransformationType::NoTranform));
        } else {
            model.objective.mul_assign(-1.0);
            model.set_sense(self.sense);
            return Ok((model, TransformationType::DidTransform));
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
