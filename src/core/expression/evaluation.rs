use super::{BiasConstraints, Expression, ExpressionEvaluation, IndexConstraints};
use crate::core::IndexByValue;
use std::ops::Mul;

impl<Index, Bias> ExpressionEvaluation<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn evaluate_sample<'a, Elem: 'a, Sample: IndexByValue<Index, Output = Elem>>(
        &self,
        sample: &'a Sample,
    ) -> Bias
    where
        // &'a Elem: Mul<Bias, Output = Bias>,
        Elem: Mul<Bias, Output = Bias>,
    {
        let mut value = self.offset;
        // Evaluate the linear term.
        for (idx, bias) in self.linear.iter() {
            value += sample.index_by_value(idx.into()) * *bias;
        }
        // Evaluate the quadratic term if it exists.
        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                value += sample.index_by_value(u) * (sample.index_by_value(v) * bias);
            }
        }
        // Evaluate the higher order term if it exists.
        if let Some(ho) = &self.higher_order {
            for (contribs, bias) in ho.iter_contrib() {
                value += *bias
                    * contribs
                        .iter()
                        .fold(Bias::one(), |acc, x| sample.index_by_value(*x) * acc);
            }
        }
        value
    }

    fn evaluate_sampleset<
        'a,
        Elem: 'a,
        Sample: IndexByValue<Index, Output = Elem> + 'a,
        SampleSet: Iterator<Item = &'a Sample> + Copy,
    >(
        &self,
        sampleset: &'a SampleSet,
    ) -> Vec<Bias>
    where
        // &'a Elem: Mul<Bias, Output = Bias>,
        Elem: Mul<Bias, Output = Bias>,
    {
        sampleset
            .map(|sample| self.evaluate_sample(sample))
            .collect()
    }
}
