use crate::passes::{
    analysis::MinValueInConstraintAnalysis,
    special::Pipeline,
    transformation::{GeToLeConstraintsPass, IntegerToBinaryPass, LeToEqConstraintsPass},
};

pub struct IneqToEqConstraintsPipeline;

impl IneqToEqConstraintsPipeline {
    pub fn new() -> Pipeline {
        Pipeline::new(
            vec![
                GeToLeConstraintsPass::new().into(),
                MinValueInConstraintAnalysis::new().into(),
                LeToEqConstraintsPass::new().into(),
                IntegerToBinaryPass::new().into(),
            ],
            Some("ineq-to-eq-constaints".to_string()),
        )
    }
}
