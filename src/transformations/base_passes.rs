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
    pyclass(name = "ActionType", module = "aqmodels")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(name = "ActionType", module = "luna_quantum")
)]
#[derive(Clone)]
pub enum ActionType {
    DidTransform,
    DidAnalysis,
    DidAnalysisTransform,
    Nothing,
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

pub type TransformationPassResult =
    Result<(Model, Option<AnalysisCacheElement>, ActionType), TransformationPassError>;

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
