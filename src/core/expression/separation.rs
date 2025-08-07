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

    // More efficient trial, but failed at book keeping
    // fn separate(&self, vrefs: &[VarRef]) -> Result<(Expression, Expression), DifferentEnvsErr> {
    //     let n = self.num_variables;
    //     let mut left = Expression::new(self.env.clone(), vec![false; n], n);
    //     let mut right = Expression::new(self.env.clone(), Vec::new(), n);
    //     right.offset = self.offset;
    //
    //     if !vrefs.iter().all(|v| v.env == self.env) {
    //         return Err(DifferentEnvsErr);
    //     }
    //
    //     let set: HashSet<VarId> = HashSet::from_iter(vrefs.iter().map(|x| x.id));
    //
    //     left.linear = Linear::with_size(self.linear.len());
    //     right.linear = self.linear.clone();
    //
    //     for k in vrefs {
    //         let idx = k.id.into();
    //         left.linear[idx] = self.linear[idx];
    //         if self.linear[idx] != 0.0 {
    //             left.active[idx] = true;
    //         }
    //         right.linear[idx] = 0.0;
    //     }
    //     right.active = right
    //         .linear
    //         .iter()
    //         .map(|(i, k)| self.active[i] && *k != 0.0)
    //         .collect();
    //
    //     if let Some(quad) = &self.quadratic {
    //         let mut left_quad = Quadratic::new(quad.len());
    //         let mut right_quad = Quadratic::new(quad.len());
    //         for (x, y, v) in quad.iter_flat() {
    //             if set.contains(&x) || set.contains(&y) {
    //                 left.active[x.0 as usize] = true;
    //                 left.active[y.0 as usize] = true;
    //                 left_quad[(x, y)] = v;
    //             } else {
    //                 right_quad[(x, y)] = v;
    //                 right.active[x.0 as usize] = true;
    //                 right.active[y.0 as usize] = true;
    //             }
    //         }
    //         right.quadratic = Some(right_quad);
    //         left.quadratic = Some(left_quad)
    //     }
    //     if let Some(ho) = &self.higher_order {
    //         let mut left_ho = HigherOrder::default();
    //         let mut right_ho = HigherOrder::default();
    //         for (xs, v) in ho.iter_contrib() {
    //             if xs.iter().any(|x| set.contains(&x)) {
    //                 left_ho[&xs] = *v;
    //                 xs.iter().for_each(|x| {
    //                     left.active[x.0 as usize] = true;
    //                 });
    //             } else {
    //                 right_ho[&xs] = *v;
    //                 xs.iter().for_each(|x| {
    //                     right.active[x.0 as usize] = true;
    //                 });
    //             }
    //         }
    //         right.higher_order = Some(right_ho);
    //         left.higher_order = Some(left_ho)
    //     }
    //
    //     Ok((left, right))
    // }
}
