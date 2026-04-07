use lunamodel_core::{
    Model, Solution,
    ops::{LmAddAssign, LmPowAssign},
};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisPass, PassContext, ReversiblePass};
use lunamodel_types::Comparator;

use crate::analysis::{MaxBias, MaxBiasAnalysis};

use super::artifact::EqualityConstraintsToQuadraticPenaltyArtifact as ECTQPArtifact;

pub struct EqualityConstraintsToQuadraticPenaltyPass {
    penalty_scaling: f64,
    req: Vec<String>,
}

impl EqualityConstraintsToQuadraticPenaltyPass {
    pub fn new(penalty_scaling: f64) -> Self {
        Self {
            penalty_scaling,
            req: vec![MaxBiasAnalysis::NAME.to_string()],
        }
    }
}

impl ReversiblePass for EqualityConstraintsToQuadraticPenaltyPass {
    type Artifact = ECTQPArtifact;

    fn name(&self) -> &str {
        "equality-constraints-to-quadratic-penalty"
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        let max_bias: &MaxBias = ctx.require_analysis(&MaxBiasAnalysis::key())?;
        let max_bias = if max_bias.val == 0.0 {
            1.0
        } else {
            max_bias.val
        };

        let constrs = model.constraints.clone();

        for (name, constr) in constrs.iter() {
            if constr.comparator != Comparator::Eq {
                return Err(LunaModelError::Internal(
                    "cannot move inequality constraints to objective. Transform inequality constraints to equality constraints first.".into(),
                ));
            }
            let mut expr = (&constr.lhs - constr.rhs)?;
            expr.pow_assign(2)?;
            model
                .objective
                .add_assign((self.penalty_scaling * max_bias * expr)?)?;
            model.constraints.remove_constraint(&name)?;
        }

        Ok(Self::Artifact {})
    }

    fn backward(_artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution> {
        Ok(solution)
    }

    fn requires(&self) -> &[String] {
        &self.req
    }
}
