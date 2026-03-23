use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{ops::utils::make_lookup, prelude::Expression, traits::TryIndex};

impl Expression {
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

    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<Bias>
    where
        for<'s> S: TryIndex<&'s str, Output = Bias, Err = LunaModelError>,
    {
        let mut lu = vec![0.0; self.env.len()];
        make_lookup(&self.env.read_arc(), sample, &mut lu)?;
        self.evaluate_sample_quick(&lu)
    }
    pub fn evaluate_sample_quick(&self, lu: &Vec<Bias>) -> LunaModelResult<Bias> {
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
