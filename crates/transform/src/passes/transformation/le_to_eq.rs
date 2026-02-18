use crate::{BasePass, Pass, TransformationPass, passes::analysis::MinValueInConstraintAnalysis};

#[derive(Debug, Clone)]
pub struct LeToEqConstraints;

impl LeToEqConstraints {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for LeToEqConstraints {
    fn name(&self) -> String {
        String::from("le-to-eq-constraints")
    }

    fn requires(&self) -> Vec<String> {
        vec![MinValueInConstraintAnalysis::new().name()]
    }
}

impl TransformationPass for LeToEqConstraints {
    fn run(
        &self,
        model: lunamodel_core::Model,
        cache: &crate::AnalysisCache,
    ) -> crate::TransformationPassResult {
        todo!()
    }

    fn backwards(
        &self,
        solution: lunamodel_core::Solution,
        cache: &crate::AnalysisCache,
    ) -> lunamodel_core::Solution {
        todo!()
    }

    fn invalidates(&self) -> Vec<String> {
        todo!()
    }
}

impl Into<Pass> for LeToEqConstraints {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
