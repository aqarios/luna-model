use num::abs;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{AnalysisKey, AnalysisPass, PassContext};

pub struct MaxBias {
    pub val: f64,
}

pub struct MaxBiasAnalysis;

impl MaxBiasAnalysis {
    pub fn new() -> Self {
        Self {}
    }
}

impl AnalysisPass for MaxBiasAnalysis {
    type Result = MaxBias;

    const NAME: &'static str = "max-bias";
    const PROVIDES: &'static str = "max-bias";

    fn key<MaxBias>() -> AnalysisKey<MaxBias> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _ctx: &PassContext) -> LunaModelResult<Self::Result> {
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

        Ok(MaxBias { val: max_val })
    }
}
