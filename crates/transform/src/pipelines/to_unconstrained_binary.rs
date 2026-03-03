use lunamodel_types::{EnumSetFromVec, Sense, Specs, Vtype};

use crate::{
    Pass,
    passes::{
        BinarySpinPass, ChangeSensePass, IntegerToBinaryPass,
        analysis::{
            CheckModelSpecsAnalysis, MaxBiasAnalysis, MinValueForConstraintAnalysis, SpecsAnalysis,
        },
        special::Pipeline,
        transformation::{
            EqualityConstraintsToQuadraticPenalty, GeToLeConstraintsPass, LeToEqConstraintsPass,
        },
    },
};

#[derive(Debug, Clone)]
pub struct ToUnconstrainedBinaryPipeline;

impl ToUnconstrainedBinaryPipeline {
    pub fn new(penalty_factor: f64) -> Pipeline {
        let requirements = Specs {
            vtypes: Some(vec![Vtype::Binary, Vtype::Spin, Vtype::Integer].to_enumset()),
            max_degree: None,
            max_constraint_degree: Some(1),
            sense: None,
            constraints: None,
            max_num_variables: None,
        };
        // let pipeline =
        Pipeline::new(
            vec![
                // Check that the requirements are fulfilled else return Error.
                CheckModelSpecsAnalysis::new(requirements).into(),
                BinarySpinPass::new(Vtype::Binary, Some("b".to_string())).into(),
                // IntegerToBinaryPass::new().into(),
                ChangeSensePass::new(Sense::Min).into(),
                SpecsAnalysis::new().into(),
                GeToLeConstraintsPass::new().into(),
                MinValueForConstraintAnalysis::new().into(),
                LeToEqConstraintsPass::new().into(),
                IntegerToBinaryPass::new().into(),
                MaxBiasAnalysis::new().into(),
                EqualityConstraintsToQuadraticPenalty::new(penalty_factor).into(),
            ],
            Some("constrained-to-unconstrained".to_string()),
        )
        // pipeline.hide_inner = true;
        // pipeline
    }
}

// #[derive(Clone, Debug)]
// struct UnconstrainedCondition;

// impl Condition for UnconstrainedCondition {
//     fn call(&self, cache: &crate::AnalysisCache) -> lunamodel_error::LunaModelResult<bool> {
//         if let Some(AnalysisCacheElement::SpecsAnalysis(specs)) = cache.get("specs") {
//             Ok(specs.constraints.unwrap() == Ctype::Unconstrained)
//         } else {
//             Err(LunaModelError::Internal("no cache entry for specs".into()))
//         }
//     }
// }
//
impl Into<Pass> for BinarySpinPass {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
