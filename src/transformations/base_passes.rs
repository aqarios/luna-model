use std::fmt::{Debug, Display};

use crate::core::{Model, Solution};

use super::{
    analysis_cache::{AnalysisCache, AnalysisCacheElement},
    errors::{AnalysisPassError, TransformationPassError},
    passes::{
        ifelse::IfElsePass, pipeline::AbstractPipeline, special::meta_analysis::MetaAnalysisPass,
    },
};

#[cfg(feature = "py")]
use crate::py_bindings::{AnyPass, IntoAnyPass};

use dyn_clone::DynClone;
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
    DidIfElse,
    DidPipeline,
    DidNothing,
}

#[cfg(feature = "py")]
pub trait Placeholder: IntoAnyPass {}

#[cfg(not(feature = "py"))]
pub trait Placeholder {}

pub trait BasePass: Debug + Placeholder {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
    // TODO fn requires_spec(&self) -> ModelSpecs
}

impl<T: BasePass> Placeholder for T {}

pub trait AnalysisPass: BasePass + DynClone {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult;

    fn map_err(&self, err: &dyn Display) -> AnalysisPassError {
        AnalysisPassError(self.name(), err.to_string())
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

pub trait TransformationPass: BasePass + DynClone {
    fn invalidates(&self) -> Vec<String> {
        Vec::new()
    }
    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult;

    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> Solution;

    fn map_err(&self, err: &dyn Display) -> TransformationPassError {
        TransformationPassError(self.name(), err.to_string())
    }

    // fn clone_box(&self) -> Box<dyn TransformationPass>;
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
dyn_clone::clone_trait_object!(AnalysisPass);

#[derive(Debug)]
pub enum Pass {
    Transformation(Box<dyn TransformationPass>),
    Analysis(Box<dyn AnalysisPass>),
    IfElse(IfElsePass),
    Pipeline(Box<dyn AbstractPipeline>),
    MetaAnalysis(Box<dyn MetaAnalysisPass>),
}

impl Pass {
    pub fn name(&self) -> String {
        match self {
            Self::Analysis(x) => x.name(),
            Self::Transformation(x) => x.name(),
            Self::IfElse(x) => x.name(),
            Self::Pipeline(x) => x.name(),
            Self::MetaAnalysis(x) => x.name(),
        }
    }

    pub fn requires(&self) -> Vec<String> {
        match self {
            Self::Analysis(x) => x.requires(),
            Self::Transformation(x) => x.requires(),
            Self::IfElse(x) => x.requires(),
            Self::Pipeline(x) => x.requires(),
            Self::MetaAnalysis(x) => x.requires(),
        }
    }
}

impl Clone for Pass {
    fn clone(&self) -> Self {
        match self {
            Self::IfElse(x) => Self::IfElse(x.clone()),
            Self::Pipeline(x) => Self::Pipeline(x.clone()),
            Self::Transformation(x) => Self::Transformation(x.clone()),
            Self::Analysis(x) => Self::Analysis(x.clone()),
            Self::MetaAnalysis(x) => Self::MetaAnalysis(x.clone()),
        }
    }
}

impl Display for Pass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transformation(x) => write!(f, "{}", x),
            Self::Analysis(x) => write!(f, "{}", x),
            Self::IfElse(x) => write!(f, "{}", x),
            Self::Pipeline(x) => write!(f, "{}", x),
            Self::MetaAnalysis(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(feature = "py")]
impl IntoAnyPass for Pass {
    fn as_anypass(&self) -> AnyPass {
        match self {
            Self::Transformation(x) => x.as_anypass(),
            Self::Analysis(x) => x.as_anypass(),
            Self::IfElse(x) => x.as_anypass(),
            Self::Pipeline(x) => x.as_anypass(),
            Self::MetaAnalysis(x) => x.as_anypass(),
        }
    }
}
