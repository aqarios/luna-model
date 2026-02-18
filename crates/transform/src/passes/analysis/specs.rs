use crate::{AnalysisCacheElement, AnalysisPass, BasePass, Pass};

#[derive(Debug, Clone)]
pub struct SpecsAnalysis {}

impl SpecsAnalysis {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for SpecsAnalysis {
    fn name(&self) -> String {
        String::from("specs")
    }
}

impl AnalysisPass for SpecsAnalysis {
    fn run(
        &self,
        model: &lunamodel_core::Model,
        _: &crate::AnalysisCache,
    ) -> crate::AnalysisPassResult {
        let specs = model.specs();
        Ok(Some(AnalysisCacheElement::SpecsAnalysis(specs)))
    }
}

impl Into<Pass> for SpecsAnalysis {
    fn into(self) -> Pass {
        Pass::Analysis(Box::new(self))
    }
}
