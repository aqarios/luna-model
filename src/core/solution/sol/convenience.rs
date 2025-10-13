use std::fmt::Display;

use super::Solution;
use crate::{errors::ComputationErr, types::Bias};

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

#[cfg_attr(
    all(feature = "py", not(feature = "lq")),
    pyclass(name = "ValueSource", module = "aqmodels._core")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(name = "ValueSource", module = "luna_quantum._core")
)]
#[derive(Debug, Clone)]
pub enum ValueSource {
    Raw,
    Obj,
}
impl Display for ValueSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => f.write_str("raw_energies"),
            Self::Obj => f.write_str("obj_values"),
        }
    }
}

// impl TryFrom<&str> for ValueToggle {
//     type Error = ParseFromStringError;
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "raw" => Ok(Self::Raw),
//             "obj" => Ok(Self::Obj),
//             _ => Err(ParseFromStringError(value.to_string())),
//         }
//     }
// }

impl Solution {
    fn get_value(&self, toggle: ValueSource) -> Result<&Vec<Bias>, ComputationErr> {
        match toggle {
            ValueSource::Raw => &self.raw_energies,
            ValueSource::Obj => &self.obj_values,
        }
        .as_ref()
        .ok_or(ComputationErr(format!("Field '{}' not evaluated.", toggle)))
    }

    pub fn cvar(&self, alpha: Bias, toggle: Option<ValueSource>) -> Result<Bias, ComputationErr> {
        // Implementation based on definition of CVaR in https://arxiv.org/pdf/1907.04769
        // alpha has to be in (0.0, 1.0]
        if !(alpha.is_finite() && alpha > 0.0 && alpha <= 1.0) {
            return Err(ComputationErr(format!("alpha not in (0, 1], is {alpha}")));
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

    pub fn expectation_value(&self, toggle: Option<ValueSource>) -> Result<Bias, ComputationErr> {
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
    ) -> Result<Bias, ComputationErr> {
        if !(beta.is_finite() && beta >= 0.0) {
            return Err(ComputationErr(format!("beta needs to be >= 0, is {beta}")));
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

    pub fn feasibility_ratio(&self) -> Result<Bias, ComputationErr> {
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
            Err(ComputationErr(format!("feasible is not set.")))
        }
    }

    pub fn highest_constraint_violations(&self) -> Result<Option<usize>, ComputationErr> {
        if let Some(constraints) = &self.constraints {
            let mut n_violations = vec![0; constraints.len()];
            for (satisfied, &count) in constraints.iter().zip(&self.counts) {
                satisfied
                    .iter()
                    .zip(&mut n_violations)
                    .filter(|(&sat, _)| !sat)
                    .for_each(|(_, n)| *n += count)
            }

            Ok(n_violations
                .iter()
                .enumerate()
                .max_by_key(|(_, &c)| c)
                .map(|(idx, _)| idx))
        } else {
            Err(ComputationErr(format!("constraints is not set.")))
        }
    }
}
