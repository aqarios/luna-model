use derive_more::Deref;
use lunamodel_transpiler::Pipeline;
use lunamodel_types::{EnumSetFromVec, Sense, Specs, Vtype};

use crate::{
    analysis::CheckModelSpecsAnalysis,
    transformation::{BinarySpinPass, ChangeSensePass, IntegerToBinaryPass},
};

#[derive(Deref)]
pub struct ToBinaryMinimizationPipeline(pub Pipeline);

impl ToBinaryMinimizationPipeline {
    pub fn new() -> Self {
        let requirements = Specs {
            vtypes: Some(vec![Vtype::Binary, Vtype::Spin, Vtype::Integer].to_enumset()),
            max_degree: None,
            max_constraint_degree: None,
            sense: None,
            constraints: None,
            max_num_variables: None,
        };
        Self(Pipeline::new(
            "to-binary-minimization".to_string(),
            vec![
                CheckModelSpecsAnalysis::new(requirements).into(),
                ChangeSensePass::new(Sense::Min).into(),
                BinarySpinPass::new(Vtype::Binary, Some("b".to_string())).into(),
                IntegerToBinaryPass::default().into(),
            ],
        ))
    }
}
