use std::fmt;

use hashbrown::HashSet;

use crate::core::{Model, Solution, Timer};

use super::{
    analysis_cache::AnalysisCache,
    base_passes::{ActionType, Pass},
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

    fn check_dependencies(&self) -> Result<(), CompilationError> {
        let mut satisfied: HashSet<String> = HashSet::new();
        for pass in self.passes.iter() {
            let required = pass.requires();
            let mut it = required.iter().filter(|&n| !satisfied.contains(n));
            if let Some(x) = it.next() {
                return Err(CompilationError(format!(
                    "Dependency issue: Pass '{}' requires '{}', which is not satisfied.",
                    pass.name(),
                    x
                )));
            }
            satisfied.insert(pass.name().to_owned());
            if let Pass::Transformation(transform) = pass {
                transform.invalidates().iter().for_each(|&x| {
                    satisfied.remove(x);
                });
            }
        }
        Ok(())
    }

    pub fn run(&self, mut model: Model) -> Result<IntermediateRepresentation, CompilationError> {
        self.check_dependencies()?;

        let mut cache = AnalysisCache::new();
        let mut execution_log = ExecutionLog::new();

        for pass in self.passes.iter() {
            let timer = Timer::start();
            let kind = match pass {
                Pass::Transformation(x) => {
                    let ret = x.run(model, &cache)?;
                    model = ret.0;
                    ret.1
                }
                Pass::Analysis(x) => {
                    let ret = x.run(&model, &mut cache)?;
                    if let Some(inner) = ret {
                        cache.insert(&x.name(), inner);
                        ActionType::DidAnalysis
                    } else {
                        ActionType::Nothing
                    }
                }
            };
            let timing = timer.stop();
            execution_log.push(pass.name(), timing, kind)
        }

        let ir = IntermediateRepresentation {
            model,
            cache,
            execution_log,
        };
        Ok(ir)
    }

    pub fn backwards(&self, mut solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        for (general_pass, log) in self.passes.iter().zip(ir.execution_log.iter()).rev() {
            match (general_pass, &log.kind) {
                (Pass::Transformation(pass), ActionType::DidAnalysis) => {
                    solution = pass.backwards(solution, &ir.cache);
                }
                _ => {}
            }
        }
        solution
    }
}

impl fmt::Display for PassManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PassManager\n")?;
        for pass in self.passes.iter() {
            let s = match pass {
                Pass::Transformation(_) => "⚙️",
                Pass::Analysis(_) => "🔎",
            };
            write!(f, "{} {}\n", s, pass.name())?;
        }
        Ok(())
    }
}
