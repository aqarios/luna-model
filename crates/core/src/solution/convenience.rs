//! Convenience statistics and helper queries for solutions.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Sense};

use crate::ValueSource;

use super::Solution;

impl Solution {
    /// Returns the selected scalar value series or an error if it is unavailable.
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

    /// Computes the conditional value at risk (CVaR) of the selected value series.
    ///
    /// The implementation follows the discrete empirical definition used for a
    /// finite set of sampled outcomes: the mean of the best `ceil(alpha * n)` of
    /// the `n` samples, where best means lowest for `Sense::Min` and highest for
    /// `Sense::Max`, and a sample with count `c` is counted `c` times.
    pub fn cvar(&self, alpha: Bias, toggle: Option<ValueSource>) -> LunaModelResult<Bias> {
        // Implementation based on definition of CVaR in https://arxiv.org/pdf/1907.04769
        // alpha has to be in (0.0, 1.0]
        if !(alpha.is_finite() && alpha > 0.0 && alpha <= 1.0) {
            return Err(LunaModelError::Computation(
                format!("alpha not in (0, 1], is {alpha}").into(),
            ));
        }

        let objs = self.get_value(toggle.unwrap_or(ValueSource::Obj))?;
        let n: usize = self.counts.iter().sum();
        if n == 0 {
            return Err(LunaModelError::Computation(
                "cannot compute CVaR of an empty solution".into(),
            ));
        }
        let m = (alpha * n as f64).ceil() as usize;

        let mut weighted: Vec<(Bias, usize)> = objs
            .iter()
            .copied()
            .zip(self.counts.iter().copied())
            .collect();
        match self.sense {
            Sense::Min => weighted.sort_unstable_by(|(a, _), (b, _)| a.total_cmp(b)),
            Sense::Max => weighted.sort_unstable_by(|(a, _), (b, _)| b.total_cmp(a)),
        }

        let mut remaining = m;
        let mut sum: Bias = 0.0;
        for (value, count) in weighted {
            let taken = count.min(remaining);
            sum += value * taken as f64;
            remaining -= taken;
            if remaining == 0 {
                break;
            }
        }

        Ok(sum / m as f64)
    }

    /// Computes the count-weighted expectation value of the selected series.
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

    /// Computes a Boltzmann-style temperature-weighted expectation value.
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

    /// Returns the fraction of total sample mass marked as feasible.
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

    /// Returns the name of the most frequently violated constraint.
    ///
    /// Returns `None` if no sample violates any constraint or the solution has no
    /// constraint information. Ties go to the alphabetically first name.
    pub fn highest_constraint_violations(&self) -> LunaModelResult<Option<String>> {
        Ok(self
            .constraints
            .iter()
            .map(|(cname, vs)| (cname, vs.iter().filter(|ok| !**ok).count()))
            .filter(|(_, n_violations)| *n_violations > 0)
            .max_by(|(a, na), (b, nb)| na.cmp(nb).then_with(|| b.cmp(a)))
            .map(|(cname, _)| cname.clone()))
    }
}
