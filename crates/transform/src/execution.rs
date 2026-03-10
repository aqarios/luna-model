use lunamodel_core::prelude::{Model, Solution, Timer};
use lunamodel_error::{LunaModelError, LunaModelResult};
use std::collections::HashSet;

use crate::base::BasePass;

use super::{
    base::{ActionType, Pass},
    cache::AnalysisCache,
    ir::IR,
    log::ExecutionLog,
    pass_manager::PassManager,
};

pub fn check_dependencies(passes: &Vec<Pass>) -> LunaModelResult<()> {
    let mut satisfied: HashSet<String> = HashSet::new();
    for pass in passes.iter() {
        let required = pass.requires();
        let mut it = required.iter().filter(|&n| !satisfied.contains(n));
        if let Some(x) = it.next() {
            return Err(LunaModelError::Compilation(
                format!(
                    "Dependency issue: Pass '{}' requires '{}', which is not satisfied.",
                    pass.name(),
                    x
                )
                .into(),
            ));
        }
        satisfied.insert(pass.name());
        if let Pass::Transformation(transform) = pass {
            transform.invalidates().iter().for_each(|x| {
                satisfied.remove(x);
            });
        }
        if let Pass::Pipeline(pipeline) = pass {
            satisfied.extend(pipeline.satisfies())
        }
    }
    Ok(())
}

pub fn run_passes(
    passes: &Vec<Pass>,
    mut model: Model,
    mut cache: AnalysisCache,
    executor: &PassManager,
) -> LunaModelResult<IR> {
    let mut execution_log = ExecutionLog::new();
    for pass in passes.iter() {
        // eprintln!("pass: {}: model is: {:?}", pass.name(), model);
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
                    // TODO@jflxb: allow DidAnalysisTransform from TransformationPass.
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
            Pass::MetaAnalysis(x) => {
                let ret = x.run(executor.passes(), &mut cache)?;
                let kind = if let Some(inner) = ret {
                    cache.insert(&x.name(), inner);
                    ActionType::DidAnalysis
                } else {
                    ActionType::DidNothing
                };
                (kind, None)
            }
            Pass::IfElse(x) => {
                let outcome = x.run(model, &cache, executor)?;
                model = outcome.ir.model;
                cache.insert(&x.name(), outcome.analysis);
                // Other passes might be dependent from the analysis inside
                // an If-Else branch. Thus we need to move the analysis contents
                // into the global cache.
                cache.insert_from(outcome.ir.cache);
                (ActionType::DidIfElse, Some(outcome.ir.execution_log))
            }
            Pass::Pipeline(x) => {
                let outcome = x.run(model, &cache, executor)?;
                model = outcome.model;
                cache.insert_from(outcome.cache);
                (ActionType::DidPipeline, Some(outcome.execution_log))
            }
        };
        let timing = timer.stop();
        execution_log.push(pass.name(), timing, kind, components)
    }

    Ok(IR {
        model,
        cache,
        execution_log,
        input_model: None,
    })
}

pub fn backwards(
    passes: &Vec<Pass>,
    mut solution: Solution,
    ir: &IR,
    log: Option<&ExecutionLog>,
) -> LunaModelResult<Solution> {
    for (general_pass, log_elem) in passes
        .iter()
        .zip(log.unwrap_or(&ir.execution_log).iter())
        .rev()
    {
        match (general_pass, &log_elem.kind) {
            (
                Pass::Transformation(pass),
                ActionType::DidTransform | ActionType::DidAnalysisTransform,
            ) => {
                solution = pass.backwards(solution, &ir.cache)?;
            }
            (Pass::IfElse(pass), ActionType::DidIfElse) => {
                if let Some(inner_log) = &log_elem.components {
                    solution = pass.backwards(solution, &ir, inner_log)?
                }
            }
            (Pass::Pipeline(pass), ActionType::DidPipeline) => {
                if let Some(inner_log) = &log_elem.components {
                    solution = pass.backwards(solution, &ir, inner_log)?
                }
            }
            _ => {}
        }
    }
    Ok(solution)
}
