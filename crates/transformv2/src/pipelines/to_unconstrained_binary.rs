use derive_more::Deref;
use lunamodel_transpiler::Pipeline;
use lunamodel_types::{EnumSetFromVec, Sense, Specs, Vtype};

use crate::{
    analysis::{
        CheckModelSpecsAnalysis, MaxBiasAnalysis, MinValueForConstraintAnalysis, SpecsAnalysis,
    },
    transformation::{
        BinarySpinPass, ChangeSensePass, EqualityConstraintsToQuadraticPenaltyPass,
        GeToLeConstraintsPass, IntegerToBinaryPass, LeToEqConstraintsPass,
    },
};

#[derive(Deref)]
pub struct ToUnconstrainedBinaryPipeline(pub Pipeline);

impl ToUnconstrainedBinaryPipeline {
    pub fn new(penalty_factor: f64) -> Self {
        let requirements = Specs {
            vtypes: Some(vec![Vtype::Binary, Vtype::Spin, Vtype::Integer].to_enumset()),
            max_degree: None,
            max_constraint_degree: Some(1),
            sense: None,
            constraints: None,
            max_num_variables: None,
        };
        Self(Pipeline::new(
            "constrained-to-unconstrained".to_string(),
            vec![
                // Check that the requirements are fulfilled else return Error.
                CheckModelSpecsAnalysis::new(requirements).into(),
                BinarySpinPass::new(Vtype::Binary, Some("b".to_string())).into(),
                // IntegerToBinaryPass::new().into(),
                ChangeSensePass::new(Sense::Min).into(),
                SpecsAnalysis::default().into(),
                GeToLeConstraintsPass::default().into(),
                MinValueForConstraintAnalysis::default().into(),
                LeToEqConstraintsPass::default().into(),
                IntegerToBinaryPass::default().into(),
                MaxBiasAnalysis::default().into(),
                EqualityConstraintsToQuadraticPenaltyPass::new(penalty_factor).into(),
            ],
        ))
    }
}

impl Into<Pipeline> for ToUnconstrainedBinaryPipeline {
    fn into(self) -> Pipeline {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_core::{
        Model,
        ops::LmAddAssign,
        prelude::{Constraint, LazyBounds},
    };
    use lunamodel_error::LunaModelResult;
    use lunamodel_transpiler::PassManager;
    use lunamodel_types::{Bound, Comparator, Vtype};

    use crate::pipelines::ToUnconstrainedBinaryPipeline;

    #[test]
    fn run_to_unconstrained_binary_pipeline() -> LunaModelResult<()> {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None)?;
        let y = model.add_var("y", Vtype::Spin, None)?;
        let z = model.add_var(
            "z",
            Vtype::Integer,
            Some(LazyBounds::new(
                Some(Bound::Bounded(0.)),
                Some(Bound::Bounded(12.)),
            )),
        )?;
        model.objective.add_assign(((&x + &y)? + &z)?)?;
        model.constraints.add_constraint(
            Constraint::new(((&x + &y)? + &z)?, 3.0, Comparator::Le, None)?,
            None,
        )?;
        model.constraints.add_constraint(
            Constraint::new(((&x - &y)? - &z)?, 0.0, Comparator::Ge, None)?,
            None,
        )?;
        model.constraints.add_constraint(
            Constraint::new((&x + &y)?, 2.0, Comparator::Eq, None)?,
            None,
        )?;

        let pm = PassManager::new().add_pipeline(ToUnconstrainedBinaryPipeline::new(10.0).into());
        let _ = pm.run(model)?;
        Ok(())
    }
}
