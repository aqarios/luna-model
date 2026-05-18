//! Analysis pass that checks a model against requested specs.

use lunamodel_core::Model;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext, PipelineStep, analysis};
use lunamodel_types::Specs;

#[analysis]
#[derive(Clone)]
pub struct CheckModelSpecsAnalysis {
    specs: Specs,
}

impl CheckModelSpecsAnalysis {
    pub fn new(specs: Specs) -> Self {
        Self { specs }
    }
}

pub struct Nothing;

impl AnalysisPass for CheckModelSpecsAnalysis {
    type Result = ();

    const PROVIDES: &'static str = "lunamodel::check-specs";

    fn name(&self) -> &str {
        "check-specs"
    }

    fn key<Nothing>() -> AnalysisKey<Nothing> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> LunaModelResult<Self::Result> {
        if !model.specs().satisfies(&self.specs) {
            return Err(LunaModelError::AnalysisPass(
                self.name().to_string(),
                "model specs do not match the requirements.".into(),
            ));
        }
        Ok(())
    }
}
