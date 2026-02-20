use lunamodel_core::{
    Model, Solution,
    ops::{LmAddAssign, LmPowAssign},
};
use lunamodel_error::LunaModelError;
use lunamodel_types::Comparator;

use crate::{
    ActionType, AnalysisCache, AnalysisCacheElement, BasePass, Pass, TransformationOutcome,
    TransformationPass, TransformationPassResult, passes::MaxBias,
};

#[derive(Debug, Clone)]
pub struct NaiveConstraintsToObjective {
    penalty: f64,
}

impl NaiveConstraintsToObjective {
    pub fn new(penalty: f64) -> Self {
        Self { penalty }
    }
}

impl BasePass for NaiveConstraintsToObjective {
    fn name(&self) -> String {
        String::from("constraints-to-objective")
    }

    fn requires(&self) -> Vec<String> {
        vec![String::from("max-bias")]
    }
}

impl TransformationPass for NaiveConstraintsToObjective {
    fn run(&self, mut model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        if let Some(AnalysisCacheElement::MaxBiasAnalysis(MaxBias { val })) = cache.get("max-bias")
        {
            let constrs = model.constraints.clone();
            let constr_len = constrs.len();
            for (name, constr) in constrs.iter() {
                if constr.comparator != Comparator::Eq {
                    return Err(LunaModelError::Internal(
                    "cannot move inequality constraints to objective. Transform inequality constraints to equality constraints using the 'GeToLeConstraintsPass' and the 'LeToEqConstraintsPass' first.".into(),
                ));
                }
                let mut expr = match constr.rhs.is_sign_positive() {
                    true => (constr.rhs - &constr.lhs)?,
                    false => (constr.rhs + &constr.lhs)?,
                };
                expr.pow_assign(2)?;
                model.objective.add_assign((self.penalty * val * expr)?)?;
                model.constraints.remove_constraint(&name)?;
            }
            let action = match constr_len == 0 {
                true => ActionType::DidNothing,
                false => ActionType::DidTransform,
            };
            Ok(TransformationOutcome::new(model, None, action))
        } else {
            return Err(LunaModelError::Internal("expected cache entry missing for ConstraintsToObjectivePass, requires MaxBiasAnalysis to be executed first.".into()));
        }
    }

    fn backwards(&self, solution: Solution, _: &AnalysisCache) -> Solution {
        solution
    }
}

impl Into<Pass> for NaiveConstraintsToObjective {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
