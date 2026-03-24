use std::fmt::Display;

use dyn_clone::DynClone;
use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::{
    base::{BasePass, Pass},
    cache::{AnalysisCache, AnalysisCacheElement},
};

use std::any::Any;

pub type MetaAnalysisPassResult = LunaModelResult<Option<AnalysisCacheElement>>;

pub trait MetaAnalysisPass: BasePass + DynClone {
    fn run(&self, passes: &[Pass], cache: &AnalysisCache) -> MetaAnalysisPassResult;

    fn map_err(&self, err: &dyn Display) -> LunaModelError {
        LunaModelError::MetaAnalysisPass(self.name(), err.to_string().into())
    }

    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
}
dyn_clone::clone_trait_object!(MetaAnalysisPass);

impl Display for dyn MetaAnalysisPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🧠 {}", self.name())?;
        Ok(())
    }
}
