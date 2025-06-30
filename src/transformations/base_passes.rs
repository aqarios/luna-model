use std::fmt::Debug;

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
    Nothing,
}

pub type TransformationPassResult = Result<(Model, ActionType), TransformationPassError>;

pub trait BasePass: Debug {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String>;
    // TODO fn requires_spec(&self) -> ModelSpecs
}

pub trait AnalysisPass: BasePass {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult;
}

pub trait TransformationPass: BasePass {
    fn invalidates(&self) -> &[&str];
    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult;

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> Solution;
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
