use std::ops::Mul;

use hashbrown::HashSet;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::VarIdx;

use crate::expression::Expression;
use crate::ops::LmAddAssign;
use crate::variable::VarRef;

impl Expression {
    pub fn separate(&self, vrefs: &[VarRef]) -> LunaModelResult<(Expression, Expression)> {
        let mut left = Expression::empty(self.env.clone());
        let mut right = Expression::empty(self.env.clone());
        right.offset = self.offset;

        if !vrefs.iter().all(|v| v.env.id() == self.env.id()) {
            return Err(LunaModelError::DifferentEnvironments);
        }

        let set: HashSet<VarIdx> = HashSet::from_iter(vrefs.iter().map(|x| x.id));

        for (x, v) in self.linear_items() {
            if v == 0.0 {
                continue;
            }
            if set.contains(&x.id) {
                // left.linear += (x, v);
                left.add_assign((&x * v)?)?;
            } else {
                // right.linear += (x, v);
                right.add_assign((&x * v)?)?;
            }
        }
        for (x, y, v) in self.quadratic_items() {
            if v == 0.0 {
                continue;
            }
            if set.contains(&x.id) || set.contains(&y.id) {
                left.add_assign(((&x * &y)? * v)?)?;
                // if let Some(q) = left.quadratic.as_mut() {
                //     *q += (x.id, y.id, v);
                // } else {
                //     let mut q = Quadratic::default();
                //     q += (x.id, y.id, v);
                //     left.quadratic = Some(q);
                // }
            } else {
                right.add_assign(((&x * &y)? * v)?)?;
                // if let Some(q) = right.quadratic.as_mut() {
                //     *q += (x.id, y.id, v);
                // } else {
                //     let mut q = Quadratic::default();
                //     q += (x.id, y.id, v);
                //     right.quadratic = Some(q);
                // }
            }
        }
        for (xs, v) in self.higher_order_items() {
            if v == 0.0 {
                continue;
            }
            let ex = (xs
                .iter()
                .fold(Expression::constant(self.env.clone(), 1.0), |n, e| {
                    n.mul(e).unwrap()
                })
                * v)?;
            dbg!(&ex);
            if xs.iter().any(|x| set.contains(&x.id)) {
                left.add_assign(ex)?;
                // if let Some(h) = left.higher_order.as_mut() {
                //     *h += (xs.as_slice(), v);
                // } else {
                //     let mut h = HigherOrder::default();
                //     h += (xs.as_slice(), v);
                //     left.higher_order = Some(h);
                // }
            } else {
                right.add_assign(ex)?;
                // if let Some(h) = right.higher_order.as_mut() {
                //     *h += (xs.as_slice(), v);
                // } else {
                //     let mut h = HigherOrder::default();
                //     h += (xs.as_slice(), v);
                //     right.higher_order = Some(h);
                // }
            }
        }

        dbg!(&self, &set, &left, &right);
        Ok((left, right))
    }
}
