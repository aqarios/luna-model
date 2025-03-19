use std::ops::Mul;

use super::{BiasConstraints, Expression, ExpressionEvaluation, IndexConstraints};

impl<Index, Bias> ExpressionEvaluation<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn evaluate_sample<'a, Elem: 'a, Sample: std::ops::Index<Index, Output = Elem>>(
        &self,
        sample: &'a Sample,
    ) -> Bias
    where
        Bias: Mul<&'a Elem, Output = Bias>,
    {
        let mut value = self.offset;
        // Evaluate the linear term.
        for (idx, bias) in self.linear.iter() {
            value += *bias * &sample[idx.into()];
        }
        // Evaluate the quadratic term if it exists.
        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                value += bias * &sample[u] * &sample[v];
            }
        }
        // Evaluate the higher order term if it exists.
        if let Some(ho) = &self.higher_order {
            for (contribs, bias) in ho.iter_contrib() {
                let mut tmp = Bias::one();
                for v in contribs.iter() {
                    tmp = tmp * &sample[*v];
                }
                value += *bias * tmp;
            }
        }
        value
    }

    fn evaluate_sampleset<
        'a,
        Elem: 'a,
        Sample: std::ops::Index<Index, Output = Elem> + 'a,
        SampleSet: Iterator<Item = &'a Sample> + Copy,
    >(
        &self,
        sampleset: &'a SampleSet,
    ) -> Vec<Bias>
    where
        Bias: Mul<&'a Elem, Output = Bias>,
    {
        sampleset
            .map(|sample| self.evaluate_sample(sample))
            .collect()
    }
}
