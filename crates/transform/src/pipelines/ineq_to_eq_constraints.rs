use crate::passes::{
    analysis::MinValueForConstraintAnalysis,
    special::Pipeline,
    transformation::{GeToLeConstraintsPass, IntegerToBinaryPass, LeToEqConstraintsPass},
};

pub struct IneqToEqConstraintsPipeline;

impl IneqToEqConstraintsPipeline {
    pub fn new() -> Pipeline {
        Pipeline::new(
            vec![
                GeToLeConstraintsPass::new().into(),
                MinValueForConstraintAnalysis::new().into(),
                LeToEqConstraintsPass::new().into(),
                IntegerToBinaryPass::new().into(),
            ],
            Some("ineq-to-eq-constaints".to_string()),
        )
    }
}
