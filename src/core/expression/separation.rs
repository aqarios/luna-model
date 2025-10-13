use std::collections::HashSet;

use crate::{
    core::{
        expression::{ExpressionBaseAdd, ExpressionBaseCreation},
        Expression, VarId, VarRef,
    },
    errors::DifferentEnvsErr,
};

pub trait Separation {
    /// Separates expression into two expressions, one that contains the provided variables and one
    /// that does not. (contains, does not contain)
    fn separate(&self, vrefs: &[VarRef]) -> Result<(Expression, Expression), DifferentEnvsErr>;
}

impl Separation for Expression {
    fn separate(&self, vrefs: &[VarRef]) -> Result<(Expression, Expression), DifferentEnvsErr> {
        let mut left = Expression::empty(self.env.clone());
        let mut right = Expression::empty(self.env.clone());
        right.offset = self.offset;

        if !vrefs.iter().all(|v| v.env == self.env) {
            return Err(DifferentEnvsErr);
        }

        let set: HashSet<VarId> = HashSet::from_iter(vrefs.iter().map(|x| x.id));

        for (x, v) in self.linear_items() {
            if v == 0.0 {
                continue;
            }
            if set.contains(&x) {
                left.add_linear(x, v);
            } else {
                right.add_linear(x, v);
            }
        }
        for (x, y, v) in self.quadratic_items() {
            if v == 0.0 {
                continue;
            }
            if set.contains(&x) || set.contains(&y) {
                left.add_quadratic(x, y, v);
            } else {
                right.add_quadratic(x, y, v);
            }
        }
        for (xs, v) in self.higher_order_items() {
            if v == 0.0 {
                continue;
            }
            if xs.iter().any(|x| set.contains(&x)) {
                left.add_higher_order(&xs, v);
            } else {
                right.add_higher_order(&xs, v);
            }
        }


        Ok((left, right))
    }
}
