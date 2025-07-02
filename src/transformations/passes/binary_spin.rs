use std::collections::HashMap;

use crate::{
    core::{expression::ExpressionBaseAdd, solution::sol::SampleCol, Model, Solution, Vtype},
    transformations::{
        analysis_cache::{AnalysisCache, AnalysisCacheElement},
        base_passes::{
            ActionType, AnalysisPass, AnalysisPassResult, BasePass, TransformationPass,
            TransformationPassResult,
        },
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
    pub prefix: Option<String>,
}

impl BinarySpinAnalysis {
    pub fn new(vtype: Vtype, prefix: Option<String>) -> Self {
        BinarySpinAnalysis { vtype, prefix }
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
                old_vtype: Vtype::Binary,
                new_vtype: vtype,
            }),
            Vtype::Binary => Ok(BinarySpinInfo {
                map,
                new_vtype: vtype,
                old_vtype: Vtype::Spin,
            }),
            x => Err(format!("Vtype '{}' not supported.", x)),
        }
    }
}

impl AnalysisPass for BinarySpinAnalysis {
    fn run(&self, model: &Model, _cache: &AnalysisCache) -> AnalysisPassResult {
        let mut cache = BinarySpinInfo::try_new(self.vtype).map_err(|x| self.map_err(&x))?;
        let pref = self.prefix.clone().unwrap_or(
            match self.vtype {
                Vtype::Binary => "x",
                Vtype::Spin => "s",
                _ => "",
            }
            .to_string(),
        );
        for x in model.environment.borrow().variables.iter() {
            match (x.vtype, self.vtype) {
                (Vtype::Binary, Vtype::Spin) => {
                    cache
                        .map
                        .insert(x.name.clone(), format!("{}_{}", pref, x.name));
                }
                (Vtype::Spin, Vtype::Binary) => {
                    cache
                        .map
                        .insert(x.name.clone(), format!("{}_{}", pref, x.name));
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
pub struct BinarySpinPass {}

impl BinarySpinPass {
    pub fn new() -> Self {
        BinarySpinPass {}
    }
}

impl BasePass for BinarySpinPass {
    fn name(&self) -> String {
        String::from("binary-spin-tr")
    }

    fn requires(&self) -> Vec<String> {
        return vec!["binary-spin".to_string()];
    }
}

impl TransformationPass for BinarySpinPass {
    #[allow(unreachable_code)]
    fn run(&self, mut model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        match cache.get("binary-spin") {
            Some(AnalysisCacheElement::BinarySpinInfoAnalysis(cache)) => {
                for (s, t) in cache.map.iter() {
                    let varref = model
                        .environment
                        .get_vref_by_name(s)
                        .map_err(|e| self.map_err(&e))?;
                    let var = model
                        .environment
                        .add_variable(t, Some(cache.new_vtype), None)
                        .map_err(|e| self.map_err(&e))?;
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
                        // This cannot be reached
                        _ => panic!(),
                    };
                    model
                        .substitute(&varref, &expr)
                        .map_err(|e| self.map_err(&e))?;
                }

                Ok((model, None, ActionType::DidTransform))
            }
            _ => Ok((model, None, ActionType::Nothing)),
        }
    }

    fn backwards(&self, mut solution: Solution, cache: &AnalysisCache) -> Solution {
        match cache.get("binary-spin") {
            Some(AnalysisCacheElement::BinarySpinInfoAnalysis(cache)) => {
                let mut rev_map = HashMap::new();
                cache.map.iter().for_each(|(k, v)| {
                    rev_map.insert(v.clone(), k.clone());
                });
                let idxs: Vec<usize> = solution
                    .variable_names
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        rev_map.get(x).map(|k| {
                            *x = k.clone();
                            i
                        })
                    })
                    .collect();
                for i in idxs.into_iter() {
                    let col = solution.samples.get_mut(i);
                    match cache.old_vtype {
                        Vtype::Spin => {
                            if let Some(SampleCol::Binary(inner)) = col {
                                solution.samples[i] = SampleCol::Spin(
                                    inner.into_iter().map(|x| 1 - 2 * (*x) as i8).collect(),
                                );
                            }
                        }
                        Vtype::Binary => {
                            if let Some(SampleCol::Spin(inner)) = col {
                                solution.samples[i] = SampleCol::Binary(
                                    inner.into_iter().map(|x| ((1 - *x) as u8) / 2).collect(),
                                );
                            }
                        }
                        _ => panic!(),
                    }
                }
            }
            _ => {}
        }
        solution
    }
}
