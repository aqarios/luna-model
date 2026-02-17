use lunamodel_core::Model;
use num::abs;

use crate::{
    base::{AnalysisPass, AnalysisPassResult, BasePass},
    cache::{AnalysisCache, AnalysisCacheElement},
};

#[derive(Debug, Clone)]
pub struct MaxBiasAnalysis {}

impl MaxBiasAnalysis {
    pub fn new() -> Self {
        MaxBiasAnalysis {}
    }
}

impl BasePass for MaxBiasAnalysis {
    fn name(&self) -> String {
        String::from("max-bias")
    }
}

#[cfg_attr(feature = "py", pyo3::pyclass(get_all))]
#[derive(Debug, Clone, Copy)]
pub struct MaxBias {
    pub val: f64,
}

impl AnalysisPass for MaxBiasAnalysis {
    fn run(&self, model: &Model, _cache: &AnalysisCache) -> AnalysisPassResult {
        let obj = &model.objective;
        let mut max_val = 0.0;

        let max_linear = obj.linear.iter().map(|(_, v)| abs(v)).fold(0.0, f64::max);
        max_val = f64::max(max_val, max_linear);

        if let Some(quad) = &obj.quadratic {
            let max_quadratic = quad.iter_flat().map(|(_, _, v)| abs(v)).fold(0.0, f64::max);
            max_val = f64::max(max_val, max_quadratic);
        }

        if let Some(ho) = &obj.higher_order {
            let max_ho = ho.iter().map(|(_, v)| abs(v)).fold(0.0, f64::max);
            max_val = f64::max(max_val, max_ho);
        }

        Ok(Some(AnalysisCacheElement::MaxBiasAnalysis(MaxBias {
            val: max_val,
        })))
    }
}
