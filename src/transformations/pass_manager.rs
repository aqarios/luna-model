use std::fmt;

use hashbrown::HashSet;

use crate::core::{Model, Solution, Timer};

use super::{
    analysis_cache::AnalysisCache,
    base_passes::{ActionType, BasePass, Pass},
    errors::CompilationError,
    intermediate_representation::{ExecutionLog, IntermediateRepresentation},
};

#[derive(Debug)]
pub struct PassManager {
    pub passes: Vec<Pass>,
}

impl PassManager {
    pub fn new(passes: Option<Vec<Pass>>) -> PassManager {
        if let Some(x) = passes {
            PassManager { passes: x }
        } else {
            PassManager { passes: Vec::new() }
        }
    }

    pub fn add_pass(&mut self, pass: Pass) {
        self.passes.push(pass);
    }

    pub fn run(&self, model: Model) -> Result<IntermediateRepresentation, CompilationError> {
        run_passes(&self.passes, model, AnalysisCache::new())
    }

    pub fn backwards(&self, solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        backwards(&self.passes, solution, ir)
    }
}

impl fmt::Display for PassManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PassManager\n")?;
        for pass in self.passes.iter() {
            let s = match pass {
                Pass::Transformation(_) => "⚙️",
                Pass::Analysis(_) => "🔎",
                Pass::IfElse(_) => "❔",
                Pass::Pipeline(_) => "🛢️",
            };
            write!(f, "{} {}\n", s, pass.name())?;
        }
        Ok(())
    }
}

pub fn run_passes(
    passes: &Vec<Pass>,
    mut model: Model,
    mut cache: AnalysisCache,
) -> Result<IntermediateRepresentation, CompilationError> {
    check_dependencies(&passes)?;
    let mut execution_log = ExecutionLog::new();
    for pass in passes.iter() {
        let timer = Timer::start();
        let (kind, components) = match pass {
            Pass::Transformation(x) => {
                let outcome = x.run(model, &cache)?;
                model = outcome.model;
                let kind = match outcome.action {
                    ActionType::DidTransform => {
                        if let Some(analysis) = outcome.analysis {
                            cache.insert(&x.name(), analysis);
                            ActionType::DidAnalysisTransform
                        } else {
                            ActionType::DidTransform
                        }
                    }
                    ActionType::DidNothing => ActionType::DidNothing,
                    _ => panic!("unexpected action type from TransformationPass!"),
                };
                (kind, None)
            }
            Pass::Analysis(x) => {
                let ret = x.run(&model, &mut cache)?;
                let kind = if let Some(inner) = ret {
                    cache.insert(&x.name(), inner);
                    ActionType::DidAnalysis
                } else {
                    ActionType::DidNothing
                };
                (kind, None)
            }
            Pass::IfElse(x) => {
                let outcome = x.run(model, &cache)?;
                model = outcome.ir.model;
                cache.insert(&x.name(), outcome.analysis);
                // Other passes might be dependent from the analysis inside
                // an If-Else branch. Thus we need to move the analysis contents
                // into the global cache.
                cache.insert_from(outcome.ir.cache);
                (ActionType::DidIfElse, Some(outcome.ir.execution_log))
            }
            Pass::Pipeline(x) => {
                let outcome = x.run(model, &cache)?;
                model = outcome.model;
                cache.insert_from(outcome.cache);
                (ActionType::DidPipeline, Some(outcome.execution_log))
            }
        };
        let timing = timer.stop();
        execution_log.push(pass.name(), timing, kind, components)
    }

    Ok(IntermediateRepresentation {
        model,
        cache,
        execution_log,
    })
}

fn check_dependencies(passes: &Vec<Pass>) -> Result<(), CompilationError> {
    let mut satisfied: HashSet<String> = HashSet::new();
    for pass in passes.iter() {
        // todo: include IfElse and Pipeline options
        let required = pass.requires();
        let mut it = required.iter().filter(|&n| !satisfied.contains(n));
        if let Some(x) = it.next() {
            return Err(CompilationError(format!(
                "Dependency issue: Pass '{}' requires '{}', which is not satisfied.",
                pass.name(),
                x
            )));
        }
        satisfied.insert(pass.name());
        if let Pass::Transformation(transform) = pass {
            transform.invalidates().iter().for_each(|x| {
                satisfied.remove(x);
            });
        }
    }
    Ok(())
}

pub fn backwards(
    passes: &Vec<Pass>,
    mut solution: Solution,
    ir: &IntermediateRepresentation,
) -> Solution {
    for (general_pass, log) in passes.iter().zip(ir.execution_log.iter()).rev() {
        match (general_pass, &log.kind) {
            (
                Pass::Transformation(pass),
                ActionType::DidTransform | ActionType::DidAnalysisTransform,
            ) => {
                solution = pass.backwards(solution, &ir.cache);
            }
            (Pass::IfElse(pass), ActionType::DidIfElse) => solution = pass.backwards(solution, &ir),
            (Pass::Pipeline(pass), ActionType::DidPipeline) => {
                solution = pass.backwards(solution, &ir)
            }
            _ => {}
        }
    }
    solution
}
