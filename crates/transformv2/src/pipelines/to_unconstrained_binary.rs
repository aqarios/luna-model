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
