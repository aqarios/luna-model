//! Pass logic for rewriting `<=` constraints into equalities.

use lunamodel_core::{Model, Solution, ops::LmSubAssign, prelude::LazyBounds};
use lunamodel_transpiler::{
    AnalysisPass, PassContext, PipelineStep, Reversible, TransformationPass, TranspileKindResult,
    transformation,
};
use lunamodel_types::{Bound, Comparator, Vtype};

use crate::{
    analysis::{MinConstraintValues, MinValueForConstraintAnalysis},
    error::TransformError,
};

use super::artifact::LeToEqConstraintsArtifact;

#[transformation]
#[derive(Clone)]
pub struct LeToEqConstraintsPass {
    req: Vec<String>,
}

impl Default for LeToEqConstraintsPass {
    fn default() -> Self {
        Self {
            req: vec![MinValueForConstraintAnalysis::PROVIDES.to_string()],
        }
    }
}

impl TransformationPass for LeToEqConstraintsPass {
    fn name(&self) -> &str {
        "le-to-eq-constraints"
    }

    fn requires(&self) -> &[String] {
        &self.req
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> TranspileKindResult<Self::Artifact> {
        let mut artifact = LeToEqConstraintsArtifact::default();
        let minvaldata: &MinConstraintValues =
            ctx.require_analysis(&MinValueForConstraintAnalysis::key())?;

        for (name, constr) in model.constraints.iter_mut() {
            if constr.comparator == Comparator::Le {
                let minval =
                    *minvaldata
                        .vals
                        .get(name)
                        .ok_or_else(|| TransformError::Transformation {
                            name: self.name().to_owned(),
                            msg: format!("cache does not contain an entry for constraint '{name}'"),
                        })?;
                let slack_var = model.environment.insert_with_fallback(
                    &format!("slack_{}", name),
                    Vtype::Integer,
                    Some(LazyBounds::new(
                        Some(minval),
                        Some(Bound::Bounded(constr.rhs)),
                    )),
                    None,
                )?;
                // a - s(minval, rhs) == 0
                constr.lhs.sub_assign(&slack_var)?;
                constr.rhs = 0.0;
                constr.comparator = Comparator::Eq;

                artifact.slackvars.push(slack_var.name()?);
            }
        }

        Ok(artifact)
    }
}

impl Reversible for LeToEqConstraintsPass {
    type Artifact = LeToEqConstraintsArtifact;

    const ID: &'static str = "luna_model::le-to-eq-constraints";

    fn backward(
        artifact: &Self::Artifact,
        mut solution: Solution,
    ) -> TranspileKindResult<Solution> {
        solution.remove_cols(&artifact.slackvars);
        solution.aggregate()?;
        Ok(solution)
    }
}
