use aqm_macros::analysis_cache;
use num::{abs, NumCast};

use crate::{
    core::Model,
    transformations::{
        analysis_cache::{AnalysisCache, AnalysisCacheElement},
        base_passes::{AnalysisPass, AnalysisPassResult, BasePass},
    },
};

#[cfg(feature = "py")]
use {
    crate::py_bindings::unwind, crate::transformations::base_passes::Pass, aqm_macros::py_pass,
    unwind_macros::unwindable,
};

#[cfg_attr(feature = "py", py_pass(pass_variant = "Analysis"))]
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

#[derive(Debug, Clone, Copy)]
#[analysis_cache]
pub struct MaxBias {
    pub val: f64,
}

impl AnalysisPass for MaxBiasAnalysis {
    fn run(&self, model: &Model, _cache: &AnalysisCache) -> AnalysisPassResult {
        let obj = &model.objective;
        let mut max_val = 0.0;

        let max_linear = obj
            .linear
            .iter()
            .map(|(_, v)| abs(NumCast::from(v).unwrap()))
            .fold(0.0, f64::max);
        max_val = f64::max(max_val, max_linear);

        if let Some(quad) = &obj.quadratic {
            let max_quadratic = quad
                .iter_flat()
                .map(|(_, _, v)| abs(NumCast::from(v).unwrap()))
                .fold(0.0, f64::max);
            max_val = f64::max(max_val, max_quadratic);
        }

        if let Some(ho) = &obj.higher_order {
            let max_ho = ho
                .iter()
                .map(|(_, &v)| abs(NumCast::from(v).unwrap()))
                .fold(0.0, f64::max);
            max_val = f64::max(max_val, max_ho);
        }

        Ok(Some(AnalysisCacheElement::MaxBiasAnalysis(MaxBias {
            val: max_val,
        })))
    }
}
