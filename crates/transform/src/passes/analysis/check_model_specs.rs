use lunamodel_types::Specs;

use crate::{AnalysisPass, BasePass, Pass};

#[derive(Debug, Clone)]
pub struct CheckModelSpecsAnalysis {
    specs: Specs,
}

impl CheckModelSpecsAnalysis {
    pub fn new(specs: Specs) -> Self {
        Self { specs }
    }
}

impl BasePass for CheckModelSpecsAnalysis {
    fn name(&self) -> String {
        String::from("check-specs")
    }
}

impl AnalysisPass for CheckModelSpecsAnalysis {
    fn run(
        &self,
        model: &lunamodel_core::Model,
        _: &crate::AnalysisCache,
    ) -> crate::AnalysisPassResult {
        if !model.specs().satisfies(&self.specs) {
            return Err(lunamodel_error::LunaModelError::AnalysisPass(
                self.name(),
                "model specs do not match the requirements.".into(),
            ));
        }
        Ok(None)
    }
}

impl Into<Pass> for CheckModelSpecsAnalysis {
    fn into(self) -> Pass {
        Pass::Analysis(Box::new(self))
    }
}
