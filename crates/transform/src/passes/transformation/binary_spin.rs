use std::{
    collections::HashMap,
    ops::{Add, Mul},
};

use lunamodel_core::{Model, Solution, solution::Column};
use lunamodel_error::LunaModelResult;
// use lunamodel_tpass::analysis_cache;
use lunamodel_types::Vtype;
use sqids::Sqids;

use crate::{
    base::{
        ActionType, BasePass, TransformationOutcome, TransformationPass, TransformationPassResult,
    },
    cache::{AnalysisCache, AnalysisCacheElement},
};

// #[cfg(feature = "py")]
// use {crate::base::Pass, lunamodel_tpass::py_pass, lunamodel_unwind::*};

#[cfg_attr(feature = "py", pyo3::pyclass(get_all))]
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
// #[cfg_attr(feature = "py", py_pass(pass_variant = "Transformation"))]
pub struct BinarySpinPass {
    pub vtype: Vtype,
    pub prefix: Option<String>,
}

impl BinarySpinPass {
    pub fn new(vtype: Vtype, prefix: Option<String>) -> Self {
        BinarySpinPass { vtype, prefix }
    }
}

impl BasePass for BinarySpinPass {
    fn name(&self) -> String {
        String::from("binary-spin")
    }
}

impl TransformationPass for BinarySpinPass {
    // #[allow(unreachable_code)]
    fn run(&self, mut model: Model, _cache: &AnalysisCache) -> TransformationPassResult {
        let mut cache = BinarySpinInfo::try_new(self.vtype).map_err(|x| self.map_err(&x))?;
        let pref = self.prefix.clone().unwrap_or(
            match self.vtype {
                Vtype::Binary => "x",
                Vtype::Spin => "s",
                _ => unreachable!("Cannot be reached."),
            }
            .to_string(),
        );

        let vars = model.environment.read_arc().vars().collect::<Vec<_>>();

        for x in vars.iter() {
            let vref_old = model.environment.get(*x);
            let mut new_name = format!(
                "{}_{}",
                pref,
                vref_old.name().map_err(|e| self.map_err(&e))?
            );
            // .lookup(&x.name())
            // .map_err(|e| self.map_err(&e))?;
            if model.environment.lookup(&new_name).is_ok() {
                // New name already exists
                let suffix = Sqids::default()
                    .encode(&[vref_old.id().into()])
                    .map_err(|e| self.map_err(&e))?;
                new_name = format!("{}_{}", new_name, suffix);
            }

            match (vref_old.vtype().map_err(|e| self.map_err(&e))?, self.vtype) {
                (Vtype::Binary, Vtype::Spin) | (Vtype::Spin, Vtype::Binary) => {
                    cache.map.insert(
                        vref_old.name().map_err(|e| self.map_err(&e))?.clone(),
                        new_name,
                    );
                }
                _ => {}
            };
        }

        if cache.map.is_empty() {
            return Ok(TransformationOutcome::new(
                model,
                None,
                ActionType::DidNothing,
            ));
        }

        for (s, t) in cache.map.iter() {
            let varref = model.environment.lookup(s).map_err(|e| self.map_err(&e))?;
            let var = model
                .environment
                .insert(t, cache.new_vtype, None)
                .map_err(|e| self.map_err(&e))?;
            let expr = match cache.new_vtype {
                Vtype::Spin => (&var).mul(-0.5)?.add(0.5)?,
                Vtype::Binary => (&var).mul(-2.0)?.add(1.0)?,
                // This cannot be reached
                _ => unreachable!("unexpected vtype"),
            };
            model
                .substitute(&varref, &expr)
                .map_err(|e| self.map_err(&e))?;
        }

        Ok(TransformationOutcome::new(
            model,
            Some(AnalysisCacheElement::BinarySpinInfoAnalysis(cache)),
            ActionType::DidTransform,
        ))
    }

    fn backwards(
        &self,
        mut solution: Solution,
        cache: &AnalysisCache,
    ) -> LunaModelResult<Solution> {
        match cache.get("binary-spin") {
            Some(AnalysisCacheElement::BinarySpinInfoAnalysis(cache)) => {
                let mut rev_map = HashMap::new();
                cache.map.iter().for_each(|(k, v)| {
                    rev_map.insert(v.clone(), k.clone());
                    match cache.old_vtype {
                        Vtype::Spin => {
                            if let Some(Column::Binary(inner)) = solution.samples.get(v) {
                                solution.samples.insert(
                                    k.to_string(),
                                    Column::spin(
                                        inner.iter().map(|x| (1 - 2 * x as i8) as f64).collect(),
                                    ),
                                );
                            }
                        }
                        Vtype::Binary => {
                            if let Some(Column::Spin(inner)) = solution.samples.get(v) {
                                solution.samples.insert(
                                    k.to_string(),
                                    Column::binary(
                                        inner
                                            .iter()
                                            .map(|x| (((1 - x) as u8) / 2) as f64)
                                            .collect(),
                                    ),
                                );
                            }
                        }
                        _ => panic!("unexpected vtype"),
                    }
                    solution.remove_col(v);
                });
            }
            _ => {}
        }
        Ok(solution)
    }
}
