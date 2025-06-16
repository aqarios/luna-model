use std::ops::Neg;

use super::{Expression, ExpressionBaseCreation};

impl Expression {
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

impl Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}
