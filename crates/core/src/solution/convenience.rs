use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;
use std::collections::HashMap;

use crate::ValueSource;

use super::Solution;

impl Solution {
    fn get_value(&self, toggle: ValueSource) -> LunaModelResult<&Vec<Bias>> {
        match toggle {
            ValueSource::Raw => &self.raw_energies,
            ValueSource::Obj => &self.obj_values,
        }
        .as_ref()
        .ok_or(LunaModelError::Computation(
            format!("Field '{}' not evaluated.", toggle).into(),
        ))
    }

    pub fn cvar(&self, alpha: Bias, toggle: Option<ValueSource>) -> LunaModelResult<Bias> {
        // Implementation based on definition of CVaR in https://arxiv.org/pdf/1907.04769
        // alpha has to be in (0.0, 1.0]
        if !(alpha.is_finite() && alpha > 0.0 && alpha <= 1.0) {
            return Err(LunaModelError::Computation(
                format!("alpha not in (0, 1], is {alpha}").into(),
            ));
        }

        let objs = self.get_value(toggle.unwrap_or(ValueSource::Obj))?;
        let k = objs.len();
        let m = (alpha * k as f64).ceil() as usize;
        let factor: f64 = 1.0 / m as f64;

        let mut objs = objs.to_vec();
        objs.select_nth_unstable_by(m - 1, |a, b| a.total_cmp(b));
        let sum: f64 = objs[..m].iter().copied().sum();

        Ok(factor * sum)
    }

    pub fn expectation_value(&self, toggle: Option<ValueSource>) -> LunaModelResult<Bias> {
        let mut weight_sum: f64 = 0.0;
        let mut weighted_sum: Bias = Bias::default();

        let obj_values = self.get_value(toggle.unwrap_or(ValueSource::Obj))?;
        for (&ov, &c) in obj_values.iter().zip(&self.counts) {
            weight_sum += c as f64;
            weighted_sum += ov * c as f64;
        }

        Ok(weighted_sum / weight_sum)
    }

    pub fn temperature_weighted(
        &self,
        beta: Bias,
        toggle: Option<ValueSource>,
    ) -> LunaModelResult<Bias> {
        if !(beta.is_finite() && beta >= 0.0) {
            return Err(LunaModelError::Computation(
                format!("beta needs to be >= 0, is {beta}").into(),
            ));
        }

        let mut weight_sum: f64 = 0.0;
        let mut weighted_sum: Bias = Bias::default();

        let obj_values = self.get_value(toggle.unwrap_or(ValueSource::Obj))?;

        for (&ov, &c) in obj_values.iter().zip(&self.counts) {
            let factor = (-beta * ov).exp() * (c as f64);
            weight_sum += factor;
            weighted_sum += ov * factor;
        }

        Ok(weighted_sum / weight_sum)
    }

    pub fn feasibility_ratio(&self) -> LunaModelResult<Bias> {
        let mut n_feasible = 0;
        let mut n_total = 0;

        if let Some(feasible) = &self.feasible {
            for (&feas, &c) in feasible.iter().zip(&self.counts) {
                if feas {
                    n_feasible += c;
                }
                n_total += c;
            }

            Ok(n_feasible as f64 / n_total as f64)
        } else {
            Err(LunaModelError::Computation(
                "feasible is not set.".to_string().into(),
            ))
        }
    }

    pub fn highest_constraint_violations(&self) -> LunaModelResult<Option<String>> {
        if !self.constraints.is_empty() {
            let n_violations: HashMap<String, usize> = self
                .constraints
                .iter()
                .map(|(cname, vs)| (cname.clone(), vs.iter().map(|e| (!*e) as usize).sum()))
                .collect();

            Ok(n_violations
                .iter()
                .max_by_key(|(_, c)| **c)
                .map(|(key, _)| key.clone()))
        } else {
            Err(LunaModelError::Computation(
                "constraints is not set.".to_string().into(),
            ))
        }
    }
}
