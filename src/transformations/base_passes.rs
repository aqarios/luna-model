use std::fmt::Debug;

use crate::core::{
    Model, Solution,
};

use super::{
    analysis_cache::AnalysisCache,
    errors::{AnalysisPassError, TransformationPassError},
};

pub type AnalysisPassResult = Result<(), AnalysisPassError>;

pub enum TransformationType {
    DidTransform,
    NoTranform,
}

pub type TransformationPassResult = Result<(Model, TransformationType), TransformationPassError>;

pub trait BasePass: Debug {
    fn name(&self) -> &str;
    fn requires(&self) -> &[&str];
    // TODO fn requires_spec(&self) -> ModelSpecs
}

pub trait AnalysisPass: BasePass {
    fn run(&self, model: &Model, cache: &mut AnalysisCache) -> AnalysisPassResult;
}

pub trait TransformationPass: BasePass
{
    fn invalidates(&self) -> &[&str];
    fn run(
        &self,
        model: Model,
        cache: &AnalysisCache,
    ) -> TransformationPassResult;

    fn backwards(
        &self,
        solution: Solution,
        cache: &AnalysisCache,
    ) -> Solution;
}

#[derive(Debug)]
pub enum Pass
{
    Transformation(Box<dyn TransformationPass>),
    Analysis(Box<dyn AnalysisPass>),
}

impl
    Pass
{
    pub fn name(&self) -> &str {
        match self {
            Pass::Analysis(x) => x.name(),
            Pass::Transformation(x) => x.name(),
        }
    }

    pub fn requires(&self) -> &[&str] {
        match self {
            Pass::Analysis(x) => x.requires(),
            Pass::Transformation(x) => x.requires(),
        }
    }
}
