use std::collections::BTreeSet;

use global_counter::primitive::exact::CounterU64;
use lunamodel_core::{Model, Solution};
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{
    PassContext, PassManager, PipelineStep, PipelineStepRequires, ReversiblePass,
};

use super::artifact::{BranchTaken, IfElseArtifact};

/// Counter to ensure multiple if-else branches can be used in the same pass.
pub static IF_ELSE_COUNTER: CounterU64 = CounterU64::new(0);

pub struct IfElsePass {
    name: String,
    predicate: fn(&Model, &PassContext) -> LunaModelResult<bool>,
    then_steps: Vec<PipelineStep>,
    else_steps: Vec<PipelineStep>,
    requires: Vec<String>,
}

impl IfElsePass {
    pub fn new(
        predicate: fn(&Model, &PassContext) -> LunaModelResult<bool>,
        then_steps: Vec<PipelineStep>,
        else_steps: Vec<PipelineStep>,
        name: Option<String>,
    ) -> Self {
        let mut set = BTreeSet::new();
        set.extend(then_steps.collect_requires());
        set.extend(else_steps.collect_requires());
        Self {
            requires: set.into_iter().collect(),
            predicate,
            then_steps,
            else_steps,
            name: name.unwrap_or_else(|| format!("if-else-{}", IF_ELSE_COUNTER.inc())),
        }
    }
}

impl ReversiblePass for IfElsePass {
    type Artifact = IfElseArtifact;

    fn name(&self) -> &str {
        &self.name
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        let cond = (self.predicate)(model, ctx)?;

        let (steps, branch) = if cond {
            (self.then_steps.clone(), BranchTaken::Then)
        } else {
            (self.else_steps.clone(), BranchTaken::Else)
        };

        let mut sub = PassManager::from_steps(steps);
        let branch_record = sub.run(model)?;
        Ok(IfElseArtifact {
            branch: branch,
            branch_record,
        })
    }

    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution> {
        artifact.branch_record.backward(solution)
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }
}
