//! Analysis pass that infers structural model specs.

use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisKey, AnalysisPass, PassContext, PipelineStep, TranspileKindResult, analysis,
};
use lunamodel_types::Specs;

#[analysis]
#[derive(Default, Clone)]
pub struct SpecsAnalysis;

impl AnalysisPass for SpecsAnalysis {
    type Result = Specs;

    const PROVIDES: &'static str = "luna_model::specs";

    fn name(&self) -> &str {
        "specs"
    }

    fn key<Specs>() -> AnalysisKey<Specs> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> TranspileKindResult<Self::Result> {
        Ok(model.specs())
    }
}
