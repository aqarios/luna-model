use crate::{BasePass, Pass, TransformationPass, passes::MaxBiasAnalysis};

#[derive(Debug, Clone)]
pub struct GeToLeConstraints;

impl GeToLeConstraints {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for GeToLeConstraints {
    fn name(&self) -> String {
        String::from("ge-to-le-constraints")
    }

    fn requires(&self) -> Vec<String> {
        vec![]
    }
}

impl TransformationPass for GeToLeConstraints {
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

impl Into<Pass> for GeToLeConstraints {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
