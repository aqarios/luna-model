use lunamodel_core::{Model, Solution, ops::LmSubAssign, prelude::LazyBounds};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisPass, PassContext, ReversiblePass};
use lunamodel_types::{Bound, Comparator, Vtype};

use crate::analysis::{MinConstraintValues, MinValueInConstraintAnalysis};

use super::artifact::LeToEqConstraintsArtifact;

pub struct LeToEqConstraintsPass {
    req: Vec<String>,
}

impl Default for LeToEqConstraintsPass {
    fn default() -> Self {
        Self {
            req: vec![MinValueInConstraintAnalysis::NAME.to_string()],
        }
    }
}

impl ReversiblePass for LeToEqConstraintsPass {
    type Artifact = LeToEqConstraintsArtifact;

    const ID: &'static str = "lunamodel::le-to-eq-constraints";

    fn name(&self) -> &str {
        "le-to-eq-constraints"
    }

    fn requires(&self) -> &[String] {
        &self.req
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        let mut artifact = LeToEqConstraintsArtifact::default();
        let minvaldata: &MinConstraintValues =
            ctx.require_analysis(&MinValueInConstraintAnalysis::key())?;

        for (name, constr) in model.constraints.iter_mut() {
            if constr.comparator == Comparator::Le {
                let minval = *minvaldata.vals.get(name).ok_or_else(|| {
                    LunaModelError::NoConstraintForKey(
                        format!("cache does not contain an entry for constraint '{name}'").into(),
                    )
                })?;
                let slack_var = model.environment.insert_with_fallback(
                    &format!("slack_{}", name),
                    Vtype::Integer,
                    Some(LazyBounds::new(
                        Some(Bound::Bounded(minval)),
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

    fn backward(artifact: &Self::Artifact, mut solution: Solution) -> LunaModelResult<Solution> {
        solution.remove_cols(&artifact.slackvars);
        solution.aggregate()?;
        Ok(solution)
    }
}
