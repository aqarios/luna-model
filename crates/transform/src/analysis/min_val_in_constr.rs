//! Analysis pass that computes minimum achievable values per constraint.

use std::collections::HashMap;

use lunamodel_core::{Model, prelude::Bounds};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext, PipelineStep, analysis};
use lunamodel_types::Bound;

#[derive(Clone, Debug)]
pub struct MinConstraintValues {
    pub vals: HashMap<String, f64>,
}

#[analysis]
#[derive(Clone, Default)]
pub struct MinValueForConstraintAnalysis;

impl AnalysisPass for MinValueForConstraintAnalysis {
    type Result = MinConstraintValues;

    const PROVIDES: &'static str = "lunamodel::min-value-for-constraint";

    fn name(&self) -> &str {
        "min-value-for-constraint"
    }

    fn key<MinConstraintValues>() -> AnalysisKey<MinConstraintValues> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> LunaModelResult<Self::Result> {
        let mut minvalues = HashMap::new();
        for (name, constr) in model.constraints.iter() {
            // Constraint is for sure linear. Let's only look at the linear
            // stuff. Since we are in a constraint the constant (offset) is zero.
            let minvalue: f64 = constr
                .lhs
                .linear_items()
                .map(|(v, bias)| {
                    let Bounds { lower, upper } = v.bounds()?;
                    // positive coef minimized at lower bound, negative at upper bound
                    match if bias >= 0.0 { lower } else { upper } {
                        Bound::Bounded(value) => Ok(bias * value),
                        Bound::Unbounded => Err(LunaModelError::Internal(
                            format!(
                                "constraint '{name}' contains variable '{}' that is unbounded \
                       in the minimizing direction; its minimum value cannot be determined.",
                                v.name()?,
                            )
                            .into(),
                        )),
                    }
                })
                .sum::<LunaModelResult<f64>>()?;
            minvalues.insert(name.clone(), minvalue);
        }

        Ok(MinConstraintValues { vals: minvalues })
    }
}
