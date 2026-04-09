use lunamodel_transpiler::AnalysisPass;

pub struct PyAnalysisPassAdapter {}

impl AnalysisPass for PyAnalysisPassAdapter {
    fn run(&self, model: &lunamodel_core::Model, ctx: &lunamodel_transpiler::PassContext) -> lunamodel_error::LunaModelResult<Self::Result> {
        todo!()
    }
}
