use crate::{
    core::{traits::FilterByMask, Sense},
    errors::ComputationErr,
    types::Bias,
};

use super::{sol::Solution, ColElement, Column};

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

    pub fn filter_samples(&self, mask: &Vec<bool>) -> Self {
        if self.n_samples != mask.len() {
            panic!(
                "Filter sample should only be called internally and provide mask with correct len"
            )
        }
        let mut sol = Self::default();
        sol.samples = self
            .samples
            .iter()
            .map(|col| match col {
                Column::Binary(b) => {
                    Column::Binary(ColElement::new(b.varid, b.data.filter_by_mask(mask)))
                }
                Column::Spin(s) => Column::Spin(ColElement::new(s.varid, s.filter_by_mask(mask))),
                Column::Integer(i) => {
                    Column::Integer(ColElement::new(i.varid, i.filter_by_mask(mask)))
                }
                Column::Real(r) => Column::Real(ColElement::new(r.varid, r.filter_by_mask(mask))),
            })
            .collect();
        sol.sense = self.sense;
        sol.timing = self.timing;
        sol.variable_names = self.variable_names.clone();
        sol.counts = self.counts.filter_by_mask(mask);
        sol.obj_values = self.obj_values.as_ref().map(|o| o.filter_by_mask(mask));
        sol.raw_energies = self.raw_energies.filter_by_mask(mask);
        sol.constraints = self.constraints.as_ref().map(|c| c.filter_by_mask(mask));
        sol.variable_bounds = self
            .variable_bounds
            .as_ref()
            .map(|b| b.filter_by_mask(mask));
        sol.feasible = self.feasible.as_ref().map(|f| f.filter_by_mask(mask));
        sol.n_samples = sol.counts.len();
        sol.ensure_best_sample_idx();
        sol
    }
}

impl Solution {
    fn ensure_best_sample_idx(&mut self) {
        match (&self.feasible, &self.obj_values) {
            (Some(f), Some(ov)) => {
                self.best_sample_idx =
                    f.iter()
                        .zip(ov)
                        .enumerate()
                        .fold(None, |acc, (idx, (&feas, &obj))| match acc {
                            None => Some(idx),
                            Some(a) => {
                                let best_obj = ov[a];
                                if feas
                                    && (self.sense == Sense::Min && obj < best_obj
                                        || self.sense == Sense::Max && obj > best_obj)
                                {
                                    Some(idx)
                                } else {
                                    acc
                                }
                            }
                        })
            }
            _ => self.best_sample_idx = None,
        }
    }
}
