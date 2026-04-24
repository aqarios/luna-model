use std::ops::Neg;

use crate::expression::Expression;

impl Neg for Expression {
    type Output = Expression;

    /// Negates every stored contribution in the expression.
    fn neg(mut self) -> Self::Output {
        self.offset = -self.offset;
        self.linear = -self.linear;
        if let Some(q) = self.quadratic {
            self.quadratic = Some(-q);
        }
        if let Some(h) = self.higher_order {
            self.higher_order = Some(-h);
        }
        self
    }
}

impl Neg for &Expression {
    type Output = Expression;

    /// Returns a negated clone of the expression.
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}
