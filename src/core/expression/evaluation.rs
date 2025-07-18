use super::{Expression, ExpressionEvaluation};
use crate::core::expression::One;
use crate::{
    core::ValueByIndex,
    types::{Bias, VarIndex},
};
use std::ops::Mul;

impl ExpressionEvaluation<VarIndex, Bias> for Expression {
    fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>, F>(
        &self,
        sample: &'a Sample,
        index_map: F,
    ) -> Bias
    where
        // &'a Elem: Mul<Bias, Output = Bias>,
        Elem: Mul<Bias, Output = Bias>,
        F: Fn(VarIndex) -> VarIndex,
    {
        // println!("eval sample -> linear = {:?}", self.linear);
        // println!("eval sample -> active = {:?}", self.active);
        // println!("eval sample -> linear.len() = {:?}", self.linear.len());

        let mut value = self.offset;
        // Evaluate the linear term.
        for (idx, bias) in self.linear.iter() {
            // println!("in linear loop of eval -> idx = {idx}");
            if self.active[idx] {
                let mapped = index_map(idx.into());
                // println!("in linear loop of eval, idx = {idx} -> mapped idx = {mapped:?}");
                value += sample.value_by_index(mapped) * *bias;
            }
        }
        // println!("eval sample -> quadra = {:?}", self.quadratic);
        // println!(
        //     "eval sample -> quadra.len() = {:?}",
        //     self.quadratic.as_ref().map(|e| e.len())
        // );
        // Evaluate the quadratic term if it exists.
        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                value +=
                    sample.value_by_index(u) * (sample.value_by_index(index_map(v.into())) * bias);
            }
        }
        // println!("eval sample -> higher = {:?}", self.higher_order);
        // println!(
        //     "eval sample -> higher.len() = {:?}",
        //     self.higher_order.as_ref().map(|e| e.len())
        // );
        // Evaluate the higher order term if it exists.
        if let Some(ho) = &self.higher_order {
            for (contribs, bias) in ho.iter_contrib() {
                value += *bias
                    * contribs.iter().fold(Bias::one(), |acc, x| {
                        sample.value_by_index(index_map(*x)) * acc
                    });
            }
        }
        value
    }

    fn evaluate_sampleset<
        'a,
        Elem: 'a,
        Sample: ValueByIndex<VarIndex, Output = Elem> + 'a,
        SampleSet: Iterator<Item = &'a Sample> + Copy,
        F,
    >(
        &self,
        sampleset: &'a SampleSet,
        index_map: F,
    ) -> Vec<Bias>
    where
        // &'a Elem: Mul<Bias, Output = Bias>,
        Elem: Mul<Bias, Output = Bias>,
        F: Fn(VarIndex) -> VarIndex,
    {
        sampleset
            .map(|sample| self.evaluate_sample(sample, &index_map))
            .collect()
    }
}
