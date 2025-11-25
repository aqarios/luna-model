use hashbrown::HashSet;
use lunamodel_types::{VarIdx, Vtype};
use std::iter::once;

use crate::variable::VarRef;

use super::Expression;

impl Expression {
    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        self.vars()
            .map(|v| self.env.read_arc()[v.id].vtype)
            .scan(HashSet::new(), |seen, item| {
                if seen.insert(item) { Some(item) } else { None }
            })
    }

    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        self.linear
            .iter()
            .map(|(idx, _)| idx)
            .chain(
                self.quadratic
                    .iter()
                    .flat_map(|q| q.iter_flat().flat_map(|(u, v, _)| once(u).chain(once(v)))),
            )
            .chain(
                self.higher_order
                    .iter()
                    .flat_map(|h| h.iter_contrib().map(|(c, _)| c.into_iter()).flatten()),
            )
            .scan(HashSet::new(), |seen, item| {
                if seen.insert(item) { Some(item) } else { None }
            })
            .map(|id| VarRef::new(id, self.env.clone()))
    }

    pub fn degree(&self) -> usize {
        let has_qterms = if let Some(q) = &self.quadratic {
            !q.is_empty()
        } else {
            false
        };
        let has_hterms = if let Some(h) = &self.higher_order {
            !h.is_empty()
        } else {
            false
        };
        match (!self.linear.is_zero(), has_qterms, has_hterms) {
            // no terms -> deg is 0, i.e. expression is constant.
            (false, false, false) => 0,
            // only linear terms -> deg is 1, i.e. expression is linear.
            (true, false, false) => 1,
            // has quadratic terms -> deg is 2, i.e. expression is quadratic.
            (_, true, false) => 2,
            // has higher_order terms -> deg is ?, i.e. expression is of higher order.
            (_, _, true) => self.higher_order.as_ref().unwrap().degree(),
        }
    }
}
