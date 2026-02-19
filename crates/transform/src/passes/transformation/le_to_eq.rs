use lunamodel_core::{ops::LmAddAssign, prelude::LazyBounds};
use lunamodel_error::LunaModelError;
use lunamodel_types::{Bound, Comparator, Vtype};

use crate::{
    ActionType, AnalysisCacheElement, BasePass, Pass, TransformationOutcome, TransformationPass,
    passes::analysis::MinValueInConstraintAnalysis,
};

#[derive(Debug, Clone)]
pub struct LeToEqConstraints;

impl LeToEqConstraints {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for LeToEqConstraints {
    fn name(&self) -> String {
        String::from("le-to-eq-constraints")
    }

    fn requires(&self) -> Vec<String> {
        vec![MinValueInConstraintAnalysis::new().name()]
    }
}

impl TransformationPass for LeToEqConstraints {
    fn run(
        &self,
        mut model: lunamodel_core::Model,
        cache: &crate::AnalysisCache,
    ) -> crate::TransformationPassResult {
        if let Some(AnalysisCacheElement::MinValueInConstraintAnalysis(minvaldata)) =
            cache.get(&MinValueInConstraintAnalysis::new().name())
        {
            let mut slackvars = Vec::new();
            for (name, constr) in model.constraints.iter_mut() {
                if constr.comparator == Comparator::Le {
                    let minval = *minvaldata.vals.get(name).ok_or_else(|| {
                        LunaModelError::NoConstraintForKey(
                            format!("cache does not contain an entry for constraint '{name}'")
                                .into(),
                        )
                    })?;
                    let slack_var = model.environment.insert_with_fallback(
                        "slack",
                        Vtype::Integer,
                        Some(LazyBounds::new(
                            Some(Bound::Bounded(minval)),
                            Some(Bound::Bounded(constr.rhs)),
                        )),
                        None,
                    )?;
                    constr.lhs.add_assign(&slack_var)?;
                    constr.rhs = 0.0;

                    slackvars.push(slack_var.name()?);
                }
            }

            let action = match slackvars.is_empty() {
                true => ActionType::DidNothing,
                false => ActionType::DidAnalysisTransform,
            };
            Ok(TransformationOutcome::new(
                model,
                Some(AnalysisCacheElement::General(slackvars)),
                action,
            ))
        } else {
            Err(LunaModelError::Internal(
                "required cache does not exist or is malformed.".into(),
            ))
        }
    }

    fn backwards(
        &self,
        mut solution: lunamodel_core::Solution,
        cache: &crate::AnalysisCache,
    ) -> lunamodel_core::Solution {
        // NOTE: dropping slack vars from the solution.
        if let Some(AnalysisCacheElement::General(slackvars)) = cache.get(&self.name()) {
            solution.remove_cols(slackvars);
        }
        solution
    }

    fn invalidates(&self) -> Vec<String> {
        self.requires()
    }
}

impl Into<Pass> for LeToEqConstraints {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
