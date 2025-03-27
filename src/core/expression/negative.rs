use std::ops::Neg;

use super::{BiasConstraints, Expression, ExpressionBaseCreation, IndexConstraints};

impl<Index, Bias> Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn negate(&self) -> Self {
        let mut out = Expression::new_from_other(&self);
        out.linear = -out.linear;
        if let Some(q) = out.quadratic {
            out.quadratic = Some(-q);
        }
        if let Some(ho) = out.higher_order {
            out.higher_order = Some(-ho);
        }
        out
    }
}

impl<Index, Bias> Neg for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl<Index, Bias> Neg for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}
