use std::collections::HashMap;

use lunamodel_core::{Expression, Model, Solution, ops::LmAddAssign, prelude::Bounds};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bound, Vtype};

use crate::{
    ActionType, AnalysisCache, AnalysisCacheElement, BasePass, Pass, TransformationOutcome,
    TransformationPass, TransformationPassResult,
};

#[derive(Debug, Clone)]
pub struct IntegerToBinaryPass;

impl IntegerToBinaryPass {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for IntegerToBinaryPass {
    fn name(&self) -> String {
        String::from("integer-to-binary")
    }
}
impl TransformationPass for IntegerToBinaryPass {
    /// IntegerToBinaryPass uses Bounded-Coefficient Encoding internally: https://arxiv.org/pdf/1706.01945
    fn run(&self, mut model: Model, _cache: &AnalysisCache) -> TransformationPassResult {
        let mut info = IntegerToBinaryInfo::new();
        let vars = model.environment.read_arc().vars().collect::<Vec<_>>();
        for v in vars.into_iter() {
            let vref = model.environment.get(v);
            if vref.vtype()? != Vtype::Integer {
                continue;
            }
            // Integer variable.
            let (new_upper, offset) = match vref.bounds()? {
                Bounds {
                    lower: Bound::Bounded(lower),
                    upper: Bound::Bounded(upper),
                } => Ok(((upper - lower) as usize, lower as usize)),
                Bounds {
                    lower: Bound::Unbounded,
                    upper: Bound::Bounded(_),
                } => Err("lower"),
                Bounds {
                    lower: Bound::Bounded(_),
                    upper: Bound::Unbounded,
                } => Err("upper"),
                Bounds {
                    lower: Bound::Unbounded,
                    upper: Bound::Unbounded,
                } => Err("lower and upper"),
            }
            .map_err(|what| {
                LunaModelError::Internal(
                    format!(
                        "Integer variable '{}' cannot be unbounded at {what} for this pass.",
                        vref.name().unwrap()
                    )
                    .into(),
                )
            })?;

            let vname = vref.name()?;
            let base_name = format!("{}_b", vname);
            // We allow our ``mu`` to be as large as it can be in standard binary encoding.
            // So we'll set it to what the largest coefficient would be if we didn't account
            // for the extra numbers that can be encoded.
            // let mu = 2f64.powf(new_upper.log2().floor() + 1.0);
            let mu = 2usize.pow(new_upper.ilog2() + 1); // already rounded down, so we can skip the `floor`.
            let coefs = bounded_coefficient_encoding(new_upper, mu);

            let mut replacement = Expression::empty(model.environment.clone());
            let mut binvarmap = HashMap::new();
            for (i, coef) in coefs.into_iter().enumerate() {
                let binref =
                    model
                        .environment
                        .insert(&format!("{base_name}{i}"), Vtype::Binary, None)?;
                binvarmap.insert(binref.name()?, coef);
                replacement.add_assign((&binref * coef)?)?;
            }
            model.substitute(&vref, &replacement)?;
            info.varmap.insert(vname.clone(), binvarmap);
            info.offsetmap.insert(vname, offset);
        }

        Ok(match info.varmap.is_empty() {
            true => TransformationOutcome::new(model, None, ActionType::DidNothing),
            false => TransformationOutcome::new(
                model,
                Some(AnalysisCacheElement::IntegerToBinaryInfoAnalysis(info)),
                ActionType::DidTransform,
            ),
        })
    }

    fn backwards(
        &self,
        mut solution: Solution,
        cache: &AnalysisCache,
    ) -> LunaModelResult<Solution> {
        match cache.get(&self.name()) {
            Some(AnalysisCacheElement::IntegerToBinaryInfoAnalysis(cache)) => {
                for (intvar, binaries) in cache.varmap.iter() {
                    let mut intcol = vec![cache.offsetmap[intvar] as f64; solution.len()];
                    for (binary_name, coef) in binaries {
                        let bincol = solution
                            .remove_col(binary_name)
                            .expect("No entry for variable '{binary_name}' in solution.");
                        for idx in 0..bincol.len() {
                            let newval = *coef as f64 * bincol[idx];
                            *intcol.get_mut(idx).unwrap() += newval;
                        }
                    }
                    solution
                        .add_col(Vtype::Integer, intvar.to_string(), intcol)
                        .expect("adding this column in the IntegerToBinaryPass.backwards must be possible.");
                }
                solution.aggregate().unwrap();
            }
            _ => {}
        }
        Ok(solution)
    }

    fn invalidates(&self) -> Vec<String> {
        vec![String::from("specs")]
    }
}

impl Into<Pass> for IntegerToBinaryPass {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}

#[cfg_attr(feature = "py", pyo3::pyclass(get_all))]
#[derive(Debug, Clone)]
pub struct IntegerToBinaryInfo {
    varmap: HashMap<String, HashMap<String, usize>>,
    offsetmap: HashMap<String, usize>,
}

impl IntegerToBinaryInfo {
    fn new() -> Self {
        Self {
            varmap: HashMap::new(),
            offsetmap: HashMap::new(),
        }
    }
}

/// Bounded Coefficient Ecoding
/// https://arxiv.org/pdf/1706.01945
/// kappa: upper bound of integer variable to be encoded
/// mu: upper bound on the coefficients of the encoding.
/// output: coefficients, length of these is the number of binary variables
/// we substitute the integer variable with.
/// Note: we loop from 0..n instead of 1..(n-1) so that we can leave out the -1 in the coefs calcs.
fn bounded_coefficient_encoding(kappa: usize, mu: usize) -> Vec<usize> {
    // let upper = 2f64.powf((mu as f64).log2().floor() + 1f64);
    let upper = 2usize.pow(mu.ilog2() + 1);
    if kappa < upper {
        // let nbits = kappa.log2().floor() as usize;
        let nbits = kappa.ilog2();
        let mut coefs = Vec::with_capacity(nbits as usize + 1);
        for i in 0..nbits {
            coefs.push(2usize.pow(i));
        }
        // let last_coef = kappa - (0..nbits).map(|e| 2usize.pow(e)).sum::<usize>();
        let last_coef = kappa - (2usize.pow(nbits) - 1);
        coefs.push(last_coef);

        coefs
    } else {
        // let rho = mu.log2().floor() as usize + 1;
        let rho = mu.ilog2() + 1;
        // let nu = kappa - (0..rho).map(|e| 2f64.powf(e as f64)).sum::<f64>();
        let nu = kappa - (0..rho).map(|e| 2usize.pow(e)).sum::<usize>();
        // let eta = (nu / mu).floor() as usize;
        let eta = nu / mu;
        // the extra one capacity is okay. In the worst case we reserved a single memory slot too much. This is fine.
        let mut coefs = Vec::with_capacity(rho as usize + eta + 1);
        for i in 0..rho {
            coefs.push(2usize.pow(i));
        }
        for _ in rho as usize..(rho as usize + eta) {
            coefs.push(mu);
        }
        let extra = nu - (eta * mu);
        if extra != 0 {
            coefs.push(extra);
        }

        coefs
    }
}

#[cfg(test)]
mod tests {
    use crate::passes::transformation::integer_to_binary::bounded_coefficient_encoding;

    #[test]
    fn bounded_coefficient_ecoding_example_1() {
        // The based on example of the paper: https://arxiv.org/pdf/1706.01945
        let kappa = 12;
        let mu = 8;
        let coefs = bounded_coefficient_encoding(kappa, mu);
        assert_eq!(vec![1, 2, 4, 5], coefs);
    }

    #[test]
    fn bounded_coefficient_ecoding_example_2() {
        // The based on example of the paper: https://arxiv.org/pdf/1706.01945
        let kappa = 20;
        let mu = 6;
        let coefs = bounded_coefficient_encoding(kappa, mu);
        assert_eq!(vec![1, 2, 4, 6, 6, 1], coefs);
    }
}
