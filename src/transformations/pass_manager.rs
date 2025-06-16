use std::fmt;

use hashbrown::HashSet;

use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    solution::AssignmentBaseTypes,
    ConcreteAssignmentTypes, ConcreteBias, ConcreteIndex, Model, Solution, Timer,
};

use super::{
    analysis_cache::AnalysisCache,
    base_passes::{Pass, TransformationType},
    errors::CompilationError,
    intermediate_representation::IntermediateRepresentation,
};

#[derive(Debug)]
pub struct PassManager<
    Index: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
> {
    pub passes: Vec<Pass<Index, Bias, AssignmentTypes>>,
}

pub type ConcretePassManager = PassManager<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>;

impl<Index: IndexConstraints, Bias: BiasConstraints, AssignmentTypes: AssignmentBaseTypes>
    PassManager<Index, Bias, AssignmentTypes>
{
    pub fn new(
        passes: Option<Vec<Pass<Index, Bias, AssignmentTypes>>>,
    ) -> PassManager<Index, Bias, AssignmentTypes> {
        if let Some(x) = passes {
            PassManager { passes: x }
        } else {
            PassManager { passes: Vec::new() }
        }
    }

    pub fn add_pass(&mut self, pass: Pass<Index, Bias, AssignmentTypes>) {
        self.passes.push(pass);
    }

    fn check_dependencies(&self) -> Result<(), CompilationError> {
        let mut satisfied: HashSet<String> = HashSet::new();
        for pass in self.passes.iter() {
            let mut it = pass.requires().iter().filter(|&&n| !satisfied.contains(n));
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

    pub fn run(
        &self,
        mut model: Model<Index, Bias>,
    ) -> Result<IntermediateRepresentation<Index, Bias>, CompilationError> {
        self.check_dependencies()?;

        let mut cache = AnalysisCache::new();

        let mut execution_log = Vec::new();

        for pass in self.passes.iter() {
            let timer = Timer::start();
            let kind = match pass {
                Pass::Transformation(x) => {
                    let ret = x.run(model, &cache)?;
                    model = ret.0;
                    Some(ret.1)
                }
                Pass::Analysis(x) => {
                    x.run(&model, &mut cache)?;
                    None
                }
            };
            let timing = timer.stop();
            execution_log.push((pass.name().to_owned(), timing, kind))
        }

        let ir = IntermediateRepresentation {
            model,
            cache,
            execution_log,
        };
        Ok(ir)
    }

    pub fn backwards(
        &self,
        mut solution: Solution<Bias, AssignmentTypes>,
        ir: &IntermediateRepresentation<Index, Bias>,
    ) -> Solution<Bias, AssignmentTypes> {
        for (general_pass, (_, _, kind)) in self.passes.iter().zip(ir.execution_log.iter()).rev() {
            match (general_pass, kind) {
                (Pass::Transformation(pass), Some(TransformationType::DidTransform)) => {
                    solution = pass.backwards(solution, &ir.cache);
                }
                _ => {}
            }
        }
        solution
    }
}

impl<Index: IndexConstraints, Bias: BiasConstraints, AssignmentTypes: AssignmentBaseTypes>
    fmt::Display for PassManager<Index, Bias, AssignmentTypes>
{
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
