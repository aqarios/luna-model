use super::{Expression, ExpressionEvaluation};
use crate::core::expression::One;
use crate::core::{Variable, Vtype};
use crate::{
    core::ValueByIndex,
    types::{Bias, VarIndex},
};
use std::ops::{Mul, Sub};

fn map_sample_value<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>, F>(
    idx: usize,
    var: &Variable,
    sample: &'a Sample,
    index_map: F,
) -> Bias
where
    Elem: Mul<Bias, Output = Bias>,
    Bias: Sub<Elem, Output = Bias>,
    F: Fn(VarIndex) -> VarIndex,
{
    if var.vtype == Vtype::InvertedBinary {
        // Get the actual non-inverted co-variable.
        let mapped = index_map(var.inverted.unwrap());
        // Value calculation changes to (1 - x) * bias.
        Bias::one() - sample.value_by_index(mapped)
    } else {
        let mapped = index_map(idx.into());
        sample.value_by_index(mapped) * Bias::one()
    }
}

impl ExpressionEvaluation<VarIndex, Bias> for Expression {
    fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>, F>(
        &self,
        sample: &'a Sample,
        index_map: F,
    ) -> Bias
    where
        Elem: Mul<Bias, Output = Bias>,
        Bias: Sub<Elem, Output = Bias>,
        F: Fn(VarIndex) -> VarIndex,
    {
        let mut value = self.offset;
        let env = self.env.access();
        // Evaluate the linear term.
        for (idx, bias) in self.linear.iter() {
            let var = &env[idx];
            value += map_sample_value(idx, &var, sample, &index_map) * bias;
        }

        // Evaluate the quadratic term if it exists.
        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                let u_var = &env[u];
                let v_var =  &env[v];
                let u_val = map_sample_value(u.into(), u_var, sample, &index_map);
                let v_val = map_sample_value(v.into(), v_var, sample, &index_map);
                value += u_val * v_val * bias;
            }
        }
        // Evaluate the higher order term if it exists.
        if let Some(ho) = &self.higher_order {
            for (contribs, bias) in ho.iter_contrib() {
                value += *bias
                    * contribs.iter().fold(Bias::one(), |acc, x| {
                        let var = &env[*x];
                        map_sample_value((*x).into(), &var, sample, &index_map) * acc
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
        Elem: Mul<Bias, Output = Bias>,
        Bias: Sub<Elem, Output = Bias>,
        F: Fn(VarIndex) -> VarIndex,
    {
        sampleset
            .map(|sample| self.evaluate_sample(sample, &index_map))
            .collect()
    }
}
