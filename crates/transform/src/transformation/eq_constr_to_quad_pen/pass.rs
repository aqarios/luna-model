//! Pass logic for equality-to-quadratic-penalty conversion.

use lunamodel_core::{
    Model, Solution,
    ops::{LmAddAssign, LmPowAssign},
};
use lunamodel_transpiler::{
    AnalysisPass, PassContext, PipelineStep, Reversible, TransformationPass, TranspileKindResult,
    transformation,
};
use lunamodel_types::Comparator;

use crate::{
    analysis::{MaxBias, MaxBiasAnalysis},
    error::TransformError,
};

use super::artifact::EqualityConstraintsToQuadraticPenaltyArtifact as ECTQPArtifact;

#[transformation]
#[derive(Clone)]
pub struct EqualityConstraintsToQuadraticPenaltyPass {
    penalty_scaling: f64,
    req: Vec<String>,
}

impl EqualityConstraintsToQuadraticPenaltyPass {
    pub fn new(penalty_scaling: f64) -> Self {
        Self {
            penalty_scaling,
            req: vec![MaxBiasAnalysis::PROVIDES.to_string()],
        }
    }

    pub fn penalty_scaling(&self) -> f64 {
        self.penalty_scaling
    }
}

impl TransformationPass for EqualityConstraintsToQuadraticPenaltyPass {
    fn name(&self) -> &str {
        "equality-constraints-to-quadratic-penalty"
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> TranspileKindResult<Self::Artifact> {
        let max_bias: &MaxBias = ctx.require_analysis(&MaxBiasAnalysis::key())?;
        let max_bias = if max_bias.val == 0.0 {
            1.0
        } else {
            max_bias.val
        };

        let constrs = model.constraints.clone();

        for (name, constr) in constrs.iter() {
            if constr.comparator != Comparator::Eq {
                return Err(TransformError::Transformation {
                    name: self.name().to_owned(),
                    msg: "cannot move inequality constraints to objective. Transform inequality constraints to equality constraints first.".into(),
                })?;
            }
            let mut expr = (&constr.lhs - constr.rhs)?;
            expr.pow_assign(2)?;
            model
                .objective
                .add_assign((self.penalty_scaling * max_bias * expr)?)?;
            model.constraints.remove_constraint(name)?;
        }

        Ok(Self::Artifact {})
    }

    fn requires(&self) -> &[String] {
        &self.req
    }
}

impl Reversible for EqualityConstraintsToQuadraticPenaltyPass {
    type Artifact = ECTQPArtifact;

    const ID: &'static str = "luna_model::equality-constraints-to-quadratic-penalty";

    fn backward(_artifact: &Self::Artifact, solution: Solution) -> TranspileKindResult<Solution> {
        Ok(solution)
    }
}
