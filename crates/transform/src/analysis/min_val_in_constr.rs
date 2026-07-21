//! Analysis pass that computes minimum achievable values per constraint.

use std::collections::HashMap;

use crate::{analysis::utils::compute_minvalue, error::TransformError};
use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisKey, AnalysisPass, PassContext, PipelineStep, TranspileKindResult, analysis,
};
use lunamodel_types::Bound;

#[derive(Clone, Debug)]
pub struct MinConstraintValues {
    pub vals: HashMap<String, Bound>,
}

#[analysis]
#[derive(Clone, Default)]
pub struct MinValueForConstraintAnalysis;

impl AnalysisPass for MinValueForConstraintAnalysis {
    type Result = MinConstraintValues;

    const PROVIDES: &'static str = "luna_model::min-value-for-constraint";

    fn name(&self) -> &str {
        "min-value-for-constraint"
    }

    fn key<MinConstraintValues>() -> AnalysisKey<MinConstraintValues> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> TranspileKindResult<Self::Result> {
        let mut minvalues = HashMap::new();
        for (name, constr) in model.constraints.iter() {
            if constr.lhs.has_quadratic() {
                return Err(TransformError::Analysis {
                    name: self.name().to_owned(),
                    msg: format!(
                        "constraint '{name}' contains quadratic terms. This is not supported, constraints must be linear."
                    ),
                })?;
            }

            if constr.lhs.has_higher_order() {
                return Err(TransformError::Analysis {
                    name: self.name().to_owned(),
                    msg: format!(
                        "constraint '{name}' contains higher-order terms. This is not supported, constraints must be linear."
                    ),
                })?;
            }

            let minvalue = compute_minvalue(constr.lhs.linear_items())?;
            minvalues.insert(name.clone(), minvalue);
        }

        Ok(MinConstraintValues { vals: minvalues })
    }
}
