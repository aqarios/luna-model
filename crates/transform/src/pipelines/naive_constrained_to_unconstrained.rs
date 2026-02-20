use lunamodel_types::{EnumSetFromVec, Specs, Vtype};

use crate::passes::{
    IntegerToBinaryPass,
    analysis::{
        CheckModelSpecsAnalysis, MaxBiasAnalysis, MinValueInConstraintAnalysis, SpecsAnalysis,
    },
    special::Pipeline,
    transformation::{GeToLeConstraintsPass, LeToEqConstraintsPass, NaiveConstraintsToObjective},
};

#[derive(Debug, Clone)]
pub struct NaiveConstrainedToUnconstrainedPipeline;

impl NaiveConstrainedToUnconstrainedPipeline {
    pub fn new(penalty_factor: f64) -> Pipeline {
        let requirements = Specs {
            vtypes: Some(vec![Vtype::Binary].to_enumset()),
            max_degree: Some(2),
            max_constraint_degree: Some(1),
            sense: None,
            constraints: None,
            max_num_variables: None,
        };
        let mut pipeline = Pipeline::new(
            vec![
                // Check that the requirements are fulfilled else return Error.
                CheckModelSpecsAnalysis::new(requirements).into(),
                SpecsAnalysis::new().into(),
                // TODO: Use an IfElsePass and only do this if it has constraints.
                // IfElsePass::new(
                //     vec!["specs".to_string()],
                //     Box::new(UnconstrainedCondition {}),
                //     Box::new(Pipeline::new(Vec::new(), None)),
                //     Box::new(IneqToEqConstraintsPipeline::new()),
                //     Some("maybe-ineq-to-eq".to_string()),
                // )
                // .into(),
                GeToLeConstraintsPass::new().into(),
                MinValueInConstraintAnalysis::new().into(),
                LeToEqConstraintsPass::new().into(),
                IntegerToBinaryPass::new().into(),
                // TODO: Use an IfElsePass and only do this if it has constraints.
                // Otherwise we can skip it.
                MaxBiasAnalysis::new().into(),
                NaiveConstraintsToObjective::new(penalty_factor).into(),
            ],
            Some("constrained-to-unconstrained".to_string()),
        );
        pipeline.hide_inner = true;
        pipeline
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
