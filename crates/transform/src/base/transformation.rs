use std::fmt::Display;

use dyn_clone::DynClone;
use lunamodel_core::prelude::{Model, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};

use super::{action::ActionType, base::BasePass};
use crate::cache::{AnalysisCache, AnalysisCacheElement};

#[cfg(feature = "py")]
use std::any::Any;

pub struct TransformationOutcome {
    pub model: Model,
    pub analysis: Option<AnalysisCacheElement>,
    pub action: ActionType,
}

impl TransformationOutcome {
    pub fn new(model: Model, analysis: Option<AnalysisCacheElement>, action: ActionType) -> Self {
        TransformationOutcome {
            model,
            analysis,
            action,
        }
    }
}
pub type TransformationPassResult = LunaModelResult<TransformationOutcome>;

pub trait TransformationPass: BasePass + DynClone {
    fn invalidates(&self) -> Vec<String> {
        Vec::new()
    }
    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult;

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> LunaModelResult<Solution>;

    fn map_err(&self, err: &dyn Display) -> LunaModelError {
        LunaModelError::TransformationPass(self.name(), err.to_string().into())
    }

    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
}

impl Display for dyn TransformationPass
where
    Self: BasePass + DynClone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "⚙️ {}", self.name())
    }
}

dyn_clone::clone_trait_object!(TransformationPass);
