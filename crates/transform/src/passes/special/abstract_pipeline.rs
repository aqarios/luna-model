use std::collections::HashSet;

use dyn_clone::DynClone;
use lunamodel_core::{Model, Solution};
use lunamodel_error::LunaModelResult;

use crate::{
    base::{BasePass, Pass},
    cache::AnalysisCache,
    ir::IR,
    log::ExecutionLog,
    pass_manager::PassManager,
};

use super::pipeline::PipelineResult;

use std::any::Any;

pub trait AbstractPipeline: BasePass + DynClone {
    fn run(&self, model: Model, cache: &AnalysisCache, executor: &PassManager) -> PipelineResult;
    fn backwards(
        &self,
        solution: Solution,
        ir: &IR,
        log: &ExecutionLog,
    ) -> LunaModelResult<Solution>;
    fn clear(&mut self);
    fn add(&mut self, pass: Pass);
    fn satisfies(&self) -> HashSet<String>;
    fn content_string(&self) -> String;
    fn len(&self) -> usize;
    fn passes(&self) -> Vec<Pass>;

    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
}
dyn_clone::clone_trait_object!(AbstractPipeline);
