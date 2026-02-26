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
pub struct EqualityConstraintsToQuadraticPenalty {
    penalty_scaling: f64,
}

impl EqualityConstraintsToQuadraticPenalty {
    pub fn new(penalty: f64) -> Self {
        Self { penalty_scaling: penalty }
    }
}

impl BasePass for EqualityConstraintsToQuadraticPenalty {
    fn name(&self) -> String {
        String::from("equality-constraints-to-quadratic-penalty")
    }

    fn requires(&self) -> Vec<String> {
        vec![String::from("max-bias")]
    }
}

impl TransformationPass for EqualityConstraintsToQuadraticPenalty {
    fn run(&self, mut model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let max_bias = if let Some(AnalysisCacheElement::MaxBiasAnalysis(MaxBias { val })) =
            cache.get("max-bias")
        {
            Ok(val)
        } else {
            Err(LunaModelError::Internal("expected cache entry 'max-bias' missing, requires 'MaxBiasAnalysis' to be executed first.".into()))
        }?;
        let constrs = model.constraints.clone();
        let constr_len = constrs.len();
        for (name, constr) in constrs.iter() {
            if constr.comparator != Comparator::Eq {
                return Err(LunaModelError::Internal(
                    "cannot move inequality constraints to objective. Transform inequality constraints to equality constraints using the 'GeToLeConstraintsPass' and the 'LeToEqConstraintsPass' first.".into(),
                ));
            }
            let mut expr = (&constr.lhs - constr.rhs)?;
            expr.pow_assign(2)?;
            model
                .objective
                .add_assign((self.penalty_scaling * max_bias * expr)?)?;
            model.constraints.remove_constraint(&name)?;
        }
        let action = match constr_len == 0 {
            true => ActionType::DidNothing,
            false => ActionType::DidTransform,
        };
        Ok(TransformationOutcome::new(model, None, action))
    }

    fn backwards(&self, solution: Solution, _: &AnalysisCache) -> Solution {
        solution
    }
}

impl Into<Pass> for EqualityConstraintsToQuadraticPenalty {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
