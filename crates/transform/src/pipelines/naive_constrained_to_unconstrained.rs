use lunamodel_types::{EnumSetFromVec, Specs, Vtype};

use super::IneqToEqConstraintsPipeline;
use crate::passes::{
    IfElsePass,
    analysis::{CheckModelSpecsAnalysis, MaxBiasAnalysis, SpecsAnalysis},
    special::{Condition, Pipeline},
    transformation::NaiveConstraintsToObjective,
};

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
        Pipeline::new(
            vec![
                // Check that the requirements are fulfilled else return Error.
                CheckModelSpecsAnalysis::new(requirements).into(),
                SpecsAnalysis::new().into(),
                IfElsePass::new(
                    vec!["specs".to_string()],
                    Box::new(UnconstrainedCondition {}),
                    Box::new(Pipeline::new(Vec::new(), None)),
                    Box::new(IneqToEqConstraintsPipeline::new()),
                    Some("maybe-ineq-to-eq".to_string()),
                )
                .into(),
                // TODO: change below to use an IfElsePass that wraps this.
                // Can save a lot of unnecessary work.

                // TODO: Use an IfElsePass and only do this if it has constraints.
                // Otherwise we can skip it.
                MaxBiasAnalysis::new().into(),
                NaiveConstraintsToObjective::new(penalty_factor).into(),
            ],
            Some("ConstrainedToUnconstrained".to_string()),
        )
    }
}

#[derive(Clone, Debug)]
struct UnconstrainedCondition;

impl Condition for UnconstrainedCondition {
    fn call(&self, cache: &crate::AnalysisCache) -> lunamodel_error::LunaModelResult<bool> {
        todo!()
    }
}
