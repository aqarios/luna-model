use std::ops::Mul;

use super::{BiasConstraints, Expression, ExpressionEvaluation, IndexConstraints};

impl<Index, Bias> ExpressionEvaluation<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn evaluate<Elem: Copy, Sample: std::ops::Index<Index, Output = Elem>>(
        &self,
        sample: &Sample,
    ) -> Bias
    where
        Bias: Mul<Elem, Output = Bias>,
    {
        let mut value = self.offset;
        // Evaluate the linear term.
        for (idx, bias) in self.linear.iter() {
            value += *bias * sample[idx.into()];
        }
        // Evaluate the quadratic term if it exists.
        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                value += bias * sample[u] * sample[v];
            }
        }
        // Evaluate the higher order term if it exists.
        if let Some(ho) = &self.higher_order {
            for (contribs, bias) in ho.iter_contrib() {
                for v in contribs.iter() {
                    value += *bias * sample[*v];
                }
            }
        }
        value
    }
}
