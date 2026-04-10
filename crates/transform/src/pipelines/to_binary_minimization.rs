use lunamodel_types::{EnumSetFromVec, Sense, Specs, Vtype};

use crate::passes::{
    BinarySpinPass, ChangeSensePass, IntegerToBinaryPass, analysis::CheckModelSpecsAnalysis,
    special::Pipeline,
};

#[derive(Debug, Clone)]
pub struct ToBinaryMinimizationPipeline;

impl ToBinaryMinimizationPipeline {
    pub fn new() -> Pipeline {
        let requirements = Specs {
            vtypes: Some(vec![Vtype::Binary, Vtype::Spin, Vtype::Integer].to_enumset()),
            max_degree: None,
            max_constraint_degree: None,
            sense: None,
            constraints: None,
            max_num_variables: None,
        };
        Pipeline::new(
            vec![
                CheckModelSpecsAnalysis::new(requirements).into(),
                ChangeSensePass::new(Sense::Min).into(),
                BinarySpinPass::new(Vtype::Binary, Some("b".to_string())).into(),
                IntegerToBinaryPass::new().into(),
            ],
            Some("to-binary-minimization".to_string()),
        )
    }
}
