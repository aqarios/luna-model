//! Pass logic for rewriting `>=` constraints into `<=` form.

use lunamodel_core::{Model, Solution, ops::LmMulAssign};
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{
    PassContext, PipelineStep, Reversible, TransformationPass, transformation,
};
use lunamodel_types::Comparator;

use super::artifact::GeToLeConstraintsArtifact as GTLCArtifact;

#[transformation]
#[derive(Default, Clone)]
pub struct GeToLeConstraintsPass;

impl TransformationPass for GeToLeConstraintsPass {
    fn name(&self) -> &str {
        "ge-to-le-constraints"
    }

    fn forward(&self, model: &mut Model, _ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        for (_, constraint) in model.constraints.iter_mut() {
            if constraint.comparator == Comparator::Ge {
                constraint.lhs.mul_assign(-1.0)?;
                constraint.rhs *= -1.0;
                constraint.comparator = Comparator::Le;
            }
        }
        Ok(Self::Artifact {})
    }
}

impl Reversible for GeToLeConstraintsPass {
    type Artifact = GTLCArtifact;

    const ID: &'static str = "luna_model::ge-to-le-constraints";

    fn backward(_artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution> {
        Ok(solution)
    }
}
