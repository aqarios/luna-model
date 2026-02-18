use std::fmt::Display;

use dyn_clone::DynClone;
use lunamodel_core::prelude::Model;
use lunamodel_error::{LunaModelError, LunaModelResult};

use super::base::BasePass;
use crate::{cache::{AnalysisCache, AnalysisCacheElement}};

#[cfg(feature = "py")]
use std::any::Any;

pub type AnalysisPassResult = LunaModelResult<Option<AnalysisCacheElement>>;

pub trait AnalysisPass: BasePass + DynClone {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult;

    fn map_err(&self, err: &dyn Display) -> LunaModelError {
        LunaModelError::AnalysisPass(self.name(), err.to_string().into())
    }

    #[cfg(feature = "py")]
    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
}

impl Display for dyn AnalysisPass
where
    Self: BasePass + DynClone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🔎 {}", self.name())
    }
}

dyn_clone::clone_trait_object!(AnalysisPass);

