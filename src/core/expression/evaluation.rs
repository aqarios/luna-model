use super::{Expression, ExpressionEvaluation};
use crate::core::expression::One;
use crate::{
    core::ValueByIndex,
    types::{Bias, VarIndex},
};
use std::ops::Mul;

impl ExpressionEvaluation<VarIndex, Bias> for Expression {
    fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>>(
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
            value += sample.value_by_index(idx.into()) * *bias;
        }
        // Evaluate the quadratic term if it exists.
        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                value += sample.value_by_index(u) * (sample.value_by_index(v) * bias);
            }
        }
        // Evaluate the higher order term if it exists.
        if let Some(ho) = &self.higher_order {
            for (contribs, bias) in ho.iter_contrib() {
                value += *bias
                    * contribs
                        .iter()
                        .fold(Bias::one(), |acc, x| sample.value_by_index(*x) * acc);
            }
        }
        value
    }

    fn evaluate_sampleset<
        'a,
        Elem: 'a,
        Sample: ValueByIndex<VarIndex, Output = Elem> + 'a,
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
