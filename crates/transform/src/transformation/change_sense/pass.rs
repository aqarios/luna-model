//! Pass logic for objective-sense normalization.

use lunamodel_core::{Model, Solution, ops::LmMulAssign};
use lunamodel_transpiler::{
    AnalysisPass, PassContext, PipelineStep, Reversible, TransformationPass, TranspileKindResult,
    transformation,
};
use lunamodel_types::Sense;

use crate::analysis::SpecsAnalysis;

use super::artifact::ChangeSensePassArtifact;

#[transformation]
#[derive(Clone)]
pub struct ChangeSensePass {
    sense: Sense,
    invalidates: Vec<String>,
}

impl ChangeSensePass {
    pub fn new(sense: Sense) -> Self {
        Self {
            sense,
            invalidates: vec![SpecsAnalysis::PROVIDES.to_string()],
        }
    }
    pub fn sense(&self) -> Sense {
        self.sense
    }
}

impl TransformationPass for ChangeSensePass {
    fn name(&self) -> &str {
        "change-sense"
    }

    fn forward(
        &self,
        model: &mut Model,
        _ctx: &PassContext,
    ) -> TranspileKindResult<Self::Artifact> {
        match model.sense == self.sense {
            true => Ok(ChangeSensePassArtifact { did_change: false }),
            false => {
                model.objective.mul_assign(-1.0)?;
                model.sense = self.sense;
                Ok(ChangeSensePassArtifact { did_change: true })
            }
        }
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates
    }
}

impl Reversible for ChangeSensePass {
    type Artifact = ChangeSensePassArtifact;

    const ID: &'static str = "luna_model::change-sense";

    fn backward(
        artifact: &Self::Artifact,
        mut solution: Solution,
    ) -> TranspileKindResult<Solution> {
        if artifact.did_change {
            solution.obj_values = solution
                .obj_values
                .map(|x| x.into_iter().map(|y| -y).collect::<Vec<_>>());
            solution.sense = !solution.sense;
        }
        Ok(solution)
    }
}
