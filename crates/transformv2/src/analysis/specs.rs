use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext};
use lunamodel_types::Specs;

#[derive(Default, Clone)]
pub struct SpecsAnalysis;

impl AnalysisPass for SpecsAnalysis {
    type Result = Specs;

    const NAME: &'static str = "specs";
    const PROVIDES: &'static str = "lunamodel::specs";

    fn key<Specs>() -> AnalysisKey<Specs> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> LunaModelResult<Self::Result> {
        Ok(model.specs())
    }
}
