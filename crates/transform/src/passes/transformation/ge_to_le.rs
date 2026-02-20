use lunamodel_core::{Model, Solution, ops::LmMulAssign};
use lunamodel_types::Comparator;

use crate::{
    ActionType, AnalysisCache, BasePass, Pass, TransformationOutcome, TransformationPass,
    TransformationPassResult,
};

#[derive(Debug, Clone)]
pub struct GeToLeConstraintsPass;

impl GeToLeConstraintsPass {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for GeToLeConstraintsPass {
    fn name(&self) -> String {
        String::from("ge-to-le-constraints")
    }

    fn requires(&self) -> Vec<String> {
        vec![]
    }
}

impl TransformationPass for GeToLeConstraintsPass {
    fn run(&self, mut model: Model, _: &AnalysisCache) -> TransformationPassResult {
        let mut did_transform: bool = false;
        for (_, constraint) in model.constraints.iter_mut() {
            if constraint.comparator == Comparator::Ge {
                constraint.lhs.mul_assign(-1.0)?;
                constraint.rhs *= -1.0;
                constraint.comparator = Comparator::Le;
                did_transform = true;
            }
        }

        let action = match did_transform {
            true => ActionType::DidTransform,
            false => ActionType::DidNothing,
        };
        TransformationPassResult::Ok(TransformationOutcome::new(model, None, action))
    }

    fn backwards(&self, solution: Solution, _: &AnalysisCache) -> Solution {
        solution
    }

    fn invalidates(&self) -> Vec<String> {
        Vec::default()
    }
}

impl Into<Pass> for GeToLeConstraintsPass {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
