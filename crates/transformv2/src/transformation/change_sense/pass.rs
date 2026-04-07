use lunamodel_core::{Model, Solution, ops::LmMulAssign};
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{AnalysisPass, PassContext, ReversiblePass};
use lunamodel_types::Sense;

use crate::analysis::SpecsAnalysis;

use super::artifact::ChangeSensePassArtifact;

pub struct ChangeSensePass {
    sense: Sense,
    inval: Vec<String>,
}

impl ChangeSensePass {
    pub fn new(sense: Sense) -> Self {
        ChangeSensePass {
            sense,
            inval: vec![SpecsAnalysis::NAME.to_string()],
        }
    }
}

impl ReversiblePass for ChangeSensePass {
    type Artifact = ChangeSensePassArtifact;

    fn name(&self) -> &str {
        "change-sense"
    }

    fn forward(&self, model: &mut Model, _ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        match model.sense == self.sense {
            true => Ok(ChangeSensePassArtifact { did_change: false }),
            false => {
                model.objective.mul_assign(-1.0)?;
                model.sense = self.sense;
                Ok(ChangeSensePassArtifact { did_change: true })
            }
        }
    }

    fn backward(artifact: &Self::Artifact, mut solution: Solution) -> LunaModelResult<Solution> {
        if artifact.did_change {
            solution.obj_values = solution
                .obj_values
                .map(|x| x.into_iter().map(|y| y * (-1.0)).collect::<Vec<_>>());
            solution.sense = !solution.sense;
        }
        Ok(solution)
    }

    fn requires(&self) -> &[String] {
        &[]
    }

    fn invalidates(&self) -> &[String] {
        &self.inval
    }
}
