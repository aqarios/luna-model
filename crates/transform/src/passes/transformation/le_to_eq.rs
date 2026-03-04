use lunamodel_core::{ops::LmSubAssign, prelude::LazyBounds};
use lunamodel_error::LunaModelError;
use lunamodel_types::{Bound, Comparator, Vtype};

use crate::{
    ActionType, AnalysisCacheElement, BasePass, Pass, TransformationOutcome, TransformationPass,
    passes::analysis::MinValueForConstraintAnalysis,
};

#[derive(Debug, Clone)]
pub struct LeToEqConstraintsPass;

impl LeToEqConstraintsPass {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for LeToEqConstraintsPass {
    fn name(&self) -> String {
        String::from("le-to-eq-constraints")
    }

    fn requires(&self) -> Vec<String> {
        vec![MinValueForConstraintAnalysis::new().name()]
    }
}

impl TransformationPass for LeToEqConstraintsPass {
    fn run(
        &self,
        mut model: lunamodel_core::Model,
        cache: &crate::AnalysisCache,
    ) -> crate::TransformationPassResult {
        if let Some(AnalysisCacheElement::MinValueInConstraintAnalysis(minvaldata)) =
            cache.get(&MinValueForConstraintAnalysis::new().name())
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
                        &format!("slack_{}", name),
                        Vtype::Integer,
                        Some(LazyBounds::new(
                            Some(Bound::Bounded(minval)),
                            // Some(Bound::Bounded(0.0)),
                            Some(Bound::Bounded(constr.rhs)),
                        )),
                        None,
                    )?;
                    // a <= b
                    // a + s == b
                    // constr.lhs.add_assign(&slack_var)?;
                    // a - s(minval, rhs) == 0
                    constr.lhs.sub_assign(&slack_var)?;
                    constr.rhs = 0.0;
                    constr.comparator = Comparator::Eq;

                    slackvars.push(slack_var.name()?);
                }
            }

            let action = match slackvars.is_empty() {
                true => ActionType::DidNothing,
                false => ActionType::DidTransform,
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
            solution.aggregate().unwrap();
        }
        solution
    }

    fn invalidates(&self) -> Vec<String> {
        let mut inv = vec![String::from("specs")];
        inv.append(&mut self.requires());
        inv
    }
}

impl Into<Pass> for LeToEqConstraintsPass {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use global_counter::primitive::exact;
    use lunamodel_core::{
        Expression, Model,
        prelude::{Constraint, ContentEquality},
    };
    use lunamodel_types::{Comparator, Vtype};

    use crate::{
        PassManager,
        passes::{
            IntegerToBinaryPass, analysis::MinValueForConstraintAnalysis,
            transformation::LeToEqConstraintsPass,
        },
    };

    #[test]
    fn simple() {
        let mut model = Model::new(None, None);
        let x = model.add_var("x", Vtype::Binary, None).unwrap();
        let y = model.add_var("y", Vtype::Binary, None).unwrap();
        let z = model.add_var("z", Vtype::Binary, None).unwrap();
        let lhs = ((&x + &y).unwrap() + &z).unwrap();
        // x + y + z <= 3
        model
            .constraints
            .add_constraint(
                Constraint::new(lhs, 3.0, Comparator::Le, Some("c0".to_string())).unwrap(),
                None,
            )
            .unwrap();
        let pm = PassManager::new(Some(vec![
            MinValueForConstraintAnalysis::new().into(),
            LeToEqConstraintsPass::new().into(),
        ]));
        let ir = pm.run(model).unwrap();
        let constr = ir.model.constraints.get("c0").unwrap();
        assert_eq!(0.0, constr.rhs);
        assert_eq!(Comparator::Eq, constr.comparator);
        let x = ir.model.environment.lookup("x").unwrap();
        let y = ir.model.environment.lookup("y").unwrap();
        let z = ir.model.environment.lookup("z").unwrap();
        let slack = ir.model.environment.lookup("slack_c0").unwrap();
        // let b1 = ir.model.environment.lookup("slack_c0_b1").unwrap();
        let expected = (((&x + &y).unwrap() + &z).unwrap() + &slack).unwrap();
        assert!(expected.equal_contents(&constr.lhs));

        // now integer to binary.
        let pm = PassManager::new(Some(vec![IntegerToBinaryPass::new().into()]));
        let ir = pm.run(ir.model).unwrap();
        let constr = ir.model.constraints.get("c0").unwrap();
        let x = ir.model.environment.lookup("x").unwrap();
        let y = ir.model.environment.lookup("y").unwrap();
        let z = ir.model.environment.lookup("z").unwrap();
        let b0 = ir.model.environment.lookup("slack_c0_b0").unwrap();
        let b1 = ir.model.environment.lookup("slack_c0_b1").unwrap();
        let expected =
            ((((&x + &y).unwrap() + &z).unwrap() + &b0).unwrap() + (&b1 * 2.0).unwrap()).unwrap();
        assert!(expected.equal_contents(&constr.lhs));
    }
}
