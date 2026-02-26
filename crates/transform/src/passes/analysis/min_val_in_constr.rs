use std::collections::HashMap;

use lunamodel_core::Model;
use lunamodel_error::LunaModelError;

use crate::{
    AnalysisCache, AnalysisCacheElement, AnalysisPass, AnalysisPassResult, BasePass, Pass,
};

#[derive(Debug, Clone)]
pub struct MinValueInConstraintAnalysis;

impl MinValueInConstraintAnalysis {
    pub fn new() -> Self {
        MinValueInConstraintAnalysis {}
    }
}

impl BasePass for MinValueInConstraintAnalysis {
    fn name(&self) -> String {
        String::from("min-value-in-constraint")
    }
}

#[cfg_attr(feature = "py", pyo3::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct MinConstraintValues {
    pub vals: HashMap<String, f64>,
}

impl AnalysisPass for MinValueInConstraintAnalysis {
    fn run(&self, model: &Model, _: &AnalysisCache) -> AnalysisPassResult {
        let mut minvalues = HashMap::new();
        for (name, constr) in model.constraints.iter() {
            // The constraint's lhs must be linear, let's make sure it is.
            if constr.lhs.has_quadratic() || constr.lhs.has_higher_order() {
                // TODO@jflxb: check with others if this makes sense to enforce here.
                // Should be handled by previous stuff but just to make sure we have
                // the error here as well.
                return Err(LunaModelError::UnsupportedOperation(
                    "all constraints must be linear for this analysis.".into(),
                ));
            }
            // Constraint is for sure linear. Let's only look at the linear
            // stuff. Since we are in a constraint the constant (offset) is zero.
            // We only care for the bias less than zero.
            let minvalue: f64 = constr
                .lhs
                .linear_items()
                .filter(|(_, bias)| *bias < 0.0)
                .map(|(_, bias)| bias)
                .sum();
            minvalues.insert(name.clone(), minvalue);
        }
        Ok(Some(AnalysisCacheElement::MinValueInConstraintAnalysis(
            MinConstraintValues { vals: minvalues },
        )))
    }
}

impl Into<Pass> for MinValueInConstraintAnalysis {
    fn into(self) -> Pass {
        Pass::Analysis(Box::new(self))
    }
}
