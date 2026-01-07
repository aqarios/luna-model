use std::ops::Index;

use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, Vtype};

use crate::prelude::Expression;

impl Expression {
    pub fn evaluate_sample<S>(&self, sample: &S) -> LunaModelResult<Bias>
    where
        for<'s> S: Index<&'s str, Output = Bias>,
    {
        let mut val = Bias::default();
        for (vs, bias) in self.items() {
            match &vs[..] {
                [] => val += bias,
                [v] => val += adjusted(sample[&v.name()?], v.vtype()?) * bias,
                [u, v] => {
                    val += adjusted(sample[&u.name()?], u.vtype()?)
                        * adjusted(sample[&v.name()?], v.vtype()?)
                        * bias
                }
                vs => {
                    let varval: LunaModelResult<f64> = vs.iter().try_fold(bias, |b, v| {
                        Ok(b * adjusted(sample[&v.name()?], v.vtype()?))
                    });
                    val += varval?;
                }
            }
        }
        Ok(val)
    }

    pub fn evaluate_sampleset<'s, Sample, S>(&self, sampleset: S) -> LunaModelResult<Vec<Bias>>
    where
        for<'v> Sample: 's + Index<&'v str, Output = Bias>,
        S: Iterator<Item = Sample>,
    {
        sampleset.map(|s| self.evaluate_sample(&s)).collect()
    }
}

fn adjusted(value: Bias, vtype: Vtype) -> Bias {
    if vtype == Vtype::InvertedBinary {
        1.0 - value
    } else {
        value
    }
}
