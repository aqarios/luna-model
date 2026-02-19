use crate::{BasePass, Pass, TransformationPass};

#[derive(Debug, Clone)]
pub struct IntegerToBinary;

impl IntegerToBinary {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for IntegerToBinary {
    fn name(&self) -> String {
        String::from("integer-to-binary")
    }
}

impl TransformationPass for IntegerToBinary {
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

impl Into<Pass> for IntegerToBinary {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
