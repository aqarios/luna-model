use std::fmt::{Debug, Display};

use crate::core::{Model, Solution};

use super::{
    analysis_cache::{AnalysisCache, AnalysisCacheElement},
    errors::{AnalysisPassError, TransformationPassError},
};

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

pub type AnalysisPassResult = Result<Option<AnalysisCacheElement>, AnalysisPassError>;

#[cfg_attr(
    all(feature = "py", not(feature = "lq")),
    pyclass(name = "ActionType", module = "aqmodels._core")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(name = "ActionType", module = "luna_quantum._core")
)]
#[derive(Clone, Debug)]
pub enum ActionType {
    DidTransform,
    DidAnalysis,
    DidAnalysisTransform,
    DidNothing,
}

pub trait BasePass: Debug {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
    // TODO fn requires_spec(&self) -> ModelSpecs
}

pub trait AnalysisPass: BasePass {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult;

    fn map_err(&self, err: &dyn Display) -> AnalysisPassError {
        AnalysisPassError(self.name(), err.to_string())
    }
}

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

pub type TransformationPassResult = Result<TransformationOutcome, TransformationPassError>;

pub trait TransformationPass: BasePass {
    fn invalidates(&self) -> Vec<String> {
        Vec::new()
    }
    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult;

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> Solution;

    fn map_err(&self, err: &dyn Display) -> TransformationPassError {
        TransformationPassError(self.name(), err.to_string())
    }
}

#[derive(Debug)]
pub enum Pass {
    Transformation(Box<dyn TransformationPass>),
    Analysis(Box<dyn AnalysisPass>),
}

impl Pass {
    pub fn name(&self) -> String {
        match self {
            Pass::Analysis(x) => x.name(),
            Pass::Transformation(x) => x.name(),
        }
    }

    pub fn requires(&self) -> Vec<String> {
        match self {
            Pass::Analysis(x) => x.requires(),
            Pass::Transformation(x) => x.requires(),
        }
    }
}
