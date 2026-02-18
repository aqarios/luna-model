use crate::passes::{
    analysis::MinValueInConstraintAnalysis,
    special::Pipeline,
    transformation::{GeToLeConstraints, IntegerToBinary, LeToEqConstraints},
};

pub struct IneqToEqConstraintsPipeline;

impl IneqToEqConstraintsPipeline {
    pub fn new() -> Pipeline {
        Pipeline::new(
            vec![
                GeToLeConstraints::new().into(),
                MinValueInConstraintAnalysis::new().into(),
                LeToEqConstraints::new().into(),
                IntegerToBinary::new().into(),
            ],
            Some("IneqToEqConstaints".to_string()),
        )
    }
}
