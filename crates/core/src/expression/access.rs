use itertools::Itertools;
use lunamodel_types::{Bias, VarIdx, Vtype};
use lunamodel_utils::unique;
use std::iter::once;

use crate::variable::VarRef;

use super::Expression;

impl Expression {
    pub fn num_vars(&self) -> usize {
        self.vars().count()
    }

    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unique(self.vars().map(|v| self.env.read_arc()[v.id].vtype))
    }

    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        unique(
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
                        .flat_map(|h| h.iter_contrib().flat_map(|(c, _)| c.into_iter())),
                ),
        )
        .map(|id| VarRef::new(id, self.env.clone()))
        .filter(|v| v.vtype().unwrap() != Vtype::InvertedBinary)
    }

    pub fn items(&self) -> impl Iterator<Item = (Vec<VarRef>, Bias)> {
        self.linear_items()
            .map(|(v, b)| (vec![v], b))
            .chain(self.quadratic_items().map(|(u, v, b)| (vec![u, v], b)))
            .chain(self.higher_order_items())
            .chain(once((Vec::new(), self.offset)))
            .filter(|(_, bias)| *bias != Bias::default())
    }

    pub fn linear_items(&self) -> impl Iterator<Item = (VarRef, Bias)> {
        self.linear
            .iter()
            .map(|(idx, bias)| (VarRef::new(idx, self.env.clone()), bias))
    }

    pub fn raw_linear_items(&self) -> impl Iterator<Item = (u32, Bias)> {
        self.linear.iter()
    }

    pub fn quadratic_items(&self) -> impl Iterator<Item = (VarRef, VarRef, Bias)> {
        self.quadratic.iter().flat_map(|q| {
            q.iter_flat().map(|(u, v, b)| {
                (
                    VarRef::new(u, self.env.clone()),
                    VarRef::new(v, self.env.clone()),
                    b,
                )
            })
        })
    }

    pub fn raw_quadratic_items(&self) -> impl Iterator<Item = (u32, u32, Bias)> {
        self.quadratic
            .iter()
            .flat_map(|q| q.iter_flat())
    }

    pub fn higher_order_items(&self) -> impl Iterator<Item = (Vec<VarRef>, Bias)> {
        self.higher_order.iter().flat_map(|q| {
            q.iter_contrib().map(|(vars, b)| {
                (
                    vars.iter()
                        .map(|&u| VarRef::new(u, self.env.clone()))
                        .collect(),
                    b,
                )
            })
        })
    }

    pub fn raw_higher_order_items(&self) -> impl Iterator<Item = (Vec<u32>, Bias)> {
        self.higher_order
            .iter()
            .flat_map(|q| q.iter_contrib())
    }

    pub fn degree(&self) -> usize {
        match (
            !self.linear.is_empty(),
            self.has_quadratic(),
            self.has_higher_order(),
        ) {
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

    pub fn is_constant(&self) -> bool {
        self.degree() == 0
    }

    pub fn has_quadratic(&self) -> bool {
        self.quadratic
            .as_ref()
            .map_or_else(|| false, |q| !q.is_empty())
    }

    pub fn has_higher_order(&self) -> bool {
        self.higher_order
            .as_ref()
            .map_or_else(|| false, |h| !h.is_empty())
    }

    pub fn linear(&self, idx: VarIdx) -> Bias {
        self.linear[idx]
    }

    pub fn quadratic(&self, u: VarIdx, v: VarIdx) -> Bias {
        self.quadratic
            .as_ref()
            .map_or_else(Bias::default, |q| q[(u, v)])
    }

    pub fn higher_order(&self, vars: &[VarIdx]) -> Bias {
        self.higher_order
            .as_ref()
            .map_or_else(Bias::default, |h| h[vars])
    }

    pub fn contains(&self, var: &VarRef) -> bool {
        self.vars().map(|v| v.id).contains(&var.id)
    }
}
