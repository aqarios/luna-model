//! Analysis pass that checks a model against requested specs.

use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisKey, AnalysisPass, PassContext, PipelineStep, TranspileKindResult, analysis,
};
use lunamodel_types::Specs;

use crate::error::TransformError;

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

    const PROVIDES: &'static str = "luna_model::check-specs";

    fn name(&self) -> &str {
        "check-specs"
    }

    fn key<Nothing>() -> AnalysisKey<Nothing> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> TranspileKindResult<Self::Result> {
        if !model.specs().satisfies(&self.specs) {
            return Err(TransformError::Analysis {
                name: self.name().to_string(),
                msg: format!(
                    "model specs do not match the requirements:\n{}",
                    model.specs().diff(&self.specs)?
                ),
            })?;
        }
        Ok(())
    }
}
