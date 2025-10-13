use crate::transformations::{
    analysis_cache::{AnalysisCache, AnalysisCacheElement},
    base_passes::{BasePass, Pass},
    errors::MetaAnalysisPassError,
};
use dyn_clone::DynClone;
use std::fmt::Display;

pub type MetaAnalysisPassResult = Result<Option<AnalysisCacheElement>, MetaAnalysisPassError>;

pub trait MetaAnalysisPass: BasePass + DynClone {
    fn run(&self, pipeline: &Vec<Pass>, cache: &AnalysisCache) -> MetaAnalysisPassResult;

    fn map_err(&self, err: &dyn Display) -> MetaAnalysisPassError {
        MetaAnalysisPassError(self.name(), err.to_string())
    }
}
dyn_clone::clone_trait_object!(MetaAnalysisPass);

impl Display for dyn MetaAnalysisPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🧠 {}", self.name())?;
        Ok(())
    }
}
