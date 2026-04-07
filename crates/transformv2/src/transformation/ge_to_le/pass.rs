use lunamodel_core::{Model, Solution, ops::LmMulAssign};
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{PassContext, ReversiblePass};
use lunamodel_types::Comparator;

use super::artifact::GeToLeConstraintsArtifact as GTLCArtifact;

#[derive(Default)]
pub struct GeToLeConstraintsPass;

impl ReversiblePass for GeToLeConstraintsPass {
    type Artifact = GTLCArtifact;

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

    fn backward(_artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution> {
        Ok(solution)
    }
}
