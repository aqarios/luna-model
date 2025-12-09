use hashbrown::HashSet;
use itertools::Itertools;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::VarIdx;

use super::Expression;
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
                left.linear += (x.id, v);
            } else {
                right.linear += (x.id, v);
            }
        }
        for (x, y, v) in self.quadratic_items() {
            if v == 0.0 {
                continue;
            }
            if set.contains(&x.id) || set.contains(&y.id) {
                left.quadratic.as_mut().map(|q| *q += (x.id, y.id, v));
            } else {
                right.quadratic.as_mut().map(|q| *q += (x.id, y.id, v));
            }
        }
        for (xs, v) in self.higher_order_items() {
            if v == 0.0 {
                continue;
            }
            let xs = xs.iter().map(|v| v.id).collect_vec();
            if xs.iter().any(|x| set.contains(x)) {
                left.higher_order.as_mut().map(|q| *q += (xs.as_slice(), v));
            } else {
                right
                    .higher_order
                    .as_mut()
                    .map(|q| *q += (xs.as_slice(), v));
            }
        }
        Ok((left, right))
    }
}
