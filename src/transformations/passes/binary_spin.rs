use std::collections::HashMap;

use crate::{
    core::{expression::ExpressionBaseAdd, Model, Solution, Variable, Vtype},
    transformations::{
        analysis_cache::{AnalysisCache, AnalysisCacheElement},
        base_passes::{
            ActionType, AnalysisPass, AnalysisPassResult, BasePass, TransformationPass,
            TransformationPassResult,
        },
        errors::{AnalysisPassError, TransformationPassError},
    },
};

#[cfg(feature = "py")]
use {
    crate::transformations::base_passes::Pass,
    aqm_macros::{analysis_cache, py_pass},
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "py", py_pass(pass_variant = "Analysis"))]
pub struct BinarySpinAnalysis {
    pub vtype: Vtype,
}

impl BinarySpinAnalysis {
    pub fn new(vtype: Vtype) -> Self {
        BinarySpinAnalysis { vtype }
    }
}

impl BasePass for BinarySpinAnalysis {
    fn name(&self) -> String {
        String::from("binary-spin")
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "py", analysis_cache)]
pub struct BinarySpinInfo {
    map: HashMap<String, String>,
    old_vtype: Vtype,
    new_vtype: Vtype,
}

impl BinarySpinInfo {
    pub fn try_new(vtype: Vtype) -> Result<Self, String> {
        let map = HashMap::new();
        match vtype {
            Vtype::Spin => Ok(BinarySpinInfo {
                map,
                old_vtype: vtype,
                new_vtype: Vtype::Binary,
            }),
            Vtype::Binary => Ok(BinarySpinInfo {
                map,
                old_vtype: vtype,
                new_vtype: Vtype::Spin,
            }),
            x => Err(format!("Vtype '{}' not supported.", x)),
        }
    }
}

impl AnalysisPass for BinarySpinAnalysis {
    fn run(&self, model: &Model, _cache: &AnalysisCache) -> AnalysisPassResult {
        let mut cache =
            BinarySpinInfo::try_new(self.vtype).map_err(|x| AnalysisPassError(self.name(), x))?;
        for x in model.environment.borrow().variables.iter() {
            match (x.vtype, self.vtype) {
                (Vtype::Binary, Vtype::Spin) | (Vtype::Spin, Vtype::Binary) => {
                    cache.map.insert(x.name.clone(), format!("s_{}", x.name));
                }
                _ => {}
            };
        }
        if cache.map.is_empty() {
            Ok(None)
        } else {
            Ok(Some(AnalysisCacheElement::BinarySpinInfoAnalysis(cache)))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
pub struct BinarySpinPass {
    pub vtype: Vtype,
}

impl BinarySpinPass {
    pub fn new(vtype: Vtype) -> Self {
        BinarySpinPass { vtype }
    }
}

impl BasePass for BinarySpinPass {
    fn name(&self) -> String {
        String::from("binary-spin")
    }

    fn requires(&self) -> Vec<String> {
        return vec!["binary-spin".to_string()];
    }
}

impl TransformationPass for BinarySpinPass {
    #[allow(unreachable_code)]
    fn run(&self, mut model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let cache: BinarySpinInfo = todo!();

        for (s, t) in cache.map.iter() {
            let varref = model
                .environment
                .get_vref_by_name(s)
                .map_err(|e| TransformationPassError(self.name(), format!("{}", e)))?;
            let var = model
                .environment
                .add_variable(t, Some(cache.new_vtype), None)
                .map_err(|e| TransformationPassError(self.name(), format!("{}", e)))?;
            let expr = match cache.new_vtype {
                Vtype::Spin => {
                    let mut e = -0.5 * var;
                    e.add_offset(0.5);
                    e
                }
                Vtype::Binary => {
                    let mut e = -2.0 * var;
                    e.add_offset(1.0);
                    e
                }
                _ => panic!(),
            };
            model
                .substitute(&varref, &expr)
                .map_err(|e| TransformationPassError(self.name(), format!("{}", e)))?;
        }

        Ok((model, ActionType::DidTransform))
    }

    fn backwards(&self, solution: Solution, _cache: &AnalysisCache) -> Solution {
        solution
    }
}
