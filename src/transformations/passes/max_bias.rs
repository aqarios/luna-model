use num::{abs, NumCast};
#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::{
    core::Model,
    transformations::{
        analysis_cache::{AnalysisCache, AnalysisCacheElement},
        base_passes::{AnalysisPass, AnalysisPassResult, BasePass},
    },
};

#[derive(Debug, Clone)]
pub struct MaxBiasAnalysis {}

impl BasePass for MaxBiasAnalysis {
    fn name(&self) -> String {
        String::from("max-bias")
    }

    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
}

#[cfg_attr(
    feature = "py",
    pyclass(get_all, name = "MaxBias", module = "aqmodels.transformations")
)]
#[derive(Debug, Clone, Copy)]
pub struct MaxBias {
    val: f64,
}

// impl AnalysisResult for MaxBias {}

impl AnalysisPass for MaxBiasAnalysis {
    fn run(&self, model: &Model, cache: &mut AnalysisCache) -> AnalysisPassResult {
        let obj = &model.objective;
        let mut max_val = 0.0;

        let max_linear = obj
            .linear
            .iter()
            .map(|(_, &v)| abs(NumCast::from(v).unwrap()))
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

        cache.insert(
            &self.name(),
            AnalysisCacheElement::MaxBiasAnalysis(MaxBias { val: max_val }),
        );
        Ok(())
    }
}
