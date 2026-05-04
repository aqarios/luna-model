//! Evaluation helpers for expressions against sample-like data.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{ops::utils::make_lookup, prelude::Expression, traits::TryIndex};

impl Expression {
    /// Evaluates the expression over an iterator of samples.
    ///
    /// Each sample is converted into a temporary lookup vector keyed by the
    /// expression environment's variable indices, then evaluated with the fast
    /// index-based path.
    pub fn evaluate_sampleset<'s, Sample, S>(&self, sampleset: S) -> LunaModelResult<Vec<Bias>>
    where
        for<'v> Sample: 's + TryIndex<&'v str, Output = Bias, Err = LunaModelError>,
        S: Iterator<Item = Sample>,
    {
        let mut res = Vec::new();

        let mut lu = vec![0.0; self.env.len()];
        for sample in sampleset {
            make_lookup(&self.env.read_arc(), &sample, &mut lu)?;
            res.push(self.evaluate_sample_quick(&lu)?);
        }
        Ok(res)
    }

    /// Evaluates the expression against a single sample accessible by variable name.
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<Bias>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        let mut lu = vec![0.0; self.env.len()];
        make_lookup(&self.env.read_arc(), sample, &mut lu)?;
        self.evaluate_sample_quick(&lu)
    }

    /// Evaluates the expression against a pre-built index lookup vector.
    ///
    /// This is the hot path used by bulk evaluation code once name-based lookup
    /// has already been resolved.
    pub fn evaluate_sample_quick(&self, lu: &[Bias]) -> LunaModelResult<Bias> {
        let mut val = self.offset;
        for (v, bias) in self.raw_linear_items() {
            val += lu[v as usize] * bias;
        }
        for (u, v, bias) in self.raw_quadratic_items() {
            val += lu[u as usize] * lu[v as usize] * bias;
        }
        for (vs, bias) in self.raw_higher_order_items() {
            let varval: LunaModelResult<f64> =
                vs.iter().try_fold(bias, |b, v| Ok(b * lu[*v as usize]));
            val += varval?;
        }
        Ok(val)
    }
}
