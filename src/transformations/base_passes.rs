use std::fmt::Debug;

use crate::core::{
    expression::{BiasConstraints, IndexConstraints}, solution::AssignmentBaseTypes, ConcreteAssignmentTypes, ConcreteBias, ConcreteIndex, Model, Solution
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

pub type TransformationPassResult<Index, Bias> =
    Result<(Model<Index, Bias>, TransformationType), TransformationPassError>;

pub trait BasePass : Debug {
    fn name(&self) -> &str;
    fn requires(&self) -> &[&str];
    // TODO fn requires_spec(&self) -> ModelSpecs
}

pub trait AnalysisPass<Index: IndexConstraints, Bias: BiasConstraints>: BasePass {
    fn run(&self, model: &Model<Index, Bias>, cache: &mut AnalysisCache) -> AnalysisPassResult;
}

pub trait TransformationPass<
    Index: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
>: BasePass
{
    fn invalidates(&self) -> &[&str];
    fn run(
        &self,
        model: Model<Index, Bias>,
        cache: &AnalysisCache,
    ) -> TransformationPassResult<Index, Bias>;

    fn backwards(
        &self,
        solution: Solution<Bias, AssignmentTypes>,
        cache: &AnalysisCache,
    ) -> Solution<Bias, AssignmentTypes>;
}

#[derive(Debug)]
pub enum Pass<Index: IndexConstraints, Bias: BiasConstraints, AssignmentTypes: AssignmentBaseTypes>
{
    Transformation(Box<dyn TransformationPass<Index, Bias, AssignmentTypes>>),
    Analysis(Box<dyn AnalysisPass<Index, Bias>>),
}

impl<Index: IndexConstraints, Bias: BiasConstraints, AssignmentTypes: AssignmentBaseTypes>
    Pass<Index, Bias, AssignmentTypes>
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

pub type ConcretePass = Pass<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>;
