use std::collections::HashMap;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext};

pub struct MinConstraintValues {
    pub vals: HashMap<String, f64>,
}

#[derive(Clone, Default)]
pub struct MinValueForConstraintAnalysis;

impl AnalysisPass for MinValueForConstraintAnalysis {
    type Result = MinConstraintValues;

    const NAME: &'static str = "min-value-for-constraint";
    const PROVIDES: &'static str = "lunamodel::min-value-for-constraint";

    fn key<MinConstraintValues>() -> AnalysisKey<MinConstraintValues> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> LunaModelResult<Self::Result> {
        let mut minvalues = HashMap::new();
        for (name, constr) in model.constraints.iter() {
            // The constraint's lhs must be linear, let's make sure it is.
            // if constr.lhs.has_quadratic() || constr.lhs.has_higher_order() {
            //     // TODO@jflxb: check with others if this makes sense to enforce here.
            //     // Should be handled by previous stuff but just to make sure we have
            //     // the error here as well.
            //     return Err(LunaModelError::UnsupportedOperation(
            //         "all constraints must be linear for this analysis.".into(),
            //     ));
            // }
            // Constraint is for sure linear. Let's only look at the linear
            // stuff. Since we are in a constraint the constant (offset) is zero.
            // We only care for the bias less than zero.
            let minvalue: f64 = constr
                .lhs
                // .linear_items()
                .items()
                .filter(|(_, bias)| *bias < 0.0)
                .map(|(_, bias)| bias)
                .sum();
            minvalues.insert(name.clone(), minvalue);
        }

        Ok(MinConstraintValues { vals: minvalues })
    }
}
