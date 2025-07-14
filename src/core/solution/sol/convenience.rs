use crate::{errors::ComputationErr, types::Bias};

use super::sol::Solution;

impl Solution {
    pub fn expectation_value(&self) -> Result<Bias, ComputationErr> {
        let mut weight_sum: f64 = 0.0;
        let mut weighted_sum: Bias = Bias::default();

        if let Some(obj_values) = &self.obj_values {
            for (&ov, &c) in obj_values.iter().zip(&self.counts) {
                weight_sum += c as f64;
                weighted_sum += ov * c as f64;
            }

            Ok(weighted_sum / weight_sum)
        } else {
            Err(ComputationErr(format!("obj_values is not set.")))
        }
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
