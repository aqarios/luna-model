//! Read-only accessors and iterators for expressions.

use itertools::Itertools;
use lunamodel_types::{Bias, VarIdx, Vtype};
use lunamodel_utils::unique;
use std::iter::once;

use crate::variable::VarRef;

use super::Expression;

impl Expression {
    /// Returns the number of distinct variables referenced by the expression.
    ///
    /// The count is based on semantic variable participation, not on raw term
    /// storage. A variable that appears in multiple terms is counted once.
    pub fn num_vars(&self) -> usize {
        self.vars().count()
    }

    /// Returns the distinct variable types used by the expression.
    ///
    /// This is derived from the environment-backed variables referenced by the
    /// expression, not from any cached metadata inside the expression itself.
    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unique(self.vars().map(|v| self.env.read_arc()[v.id].vtype))
    }

    /// Iterates over the distinct variables referenced by the expression.
    ///
    /// The iterator rebuilds [`VarRef`] values from the raw term storage and
    /// filters out inverted binary helper variables, which are treated as
    /// implementation details in most higher-level workflows.
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

    /// Iterates over all non-zero contributions as `(variables, bias)` pairs.
    ///
    /// Linear, quadratic, and higher-order terms are normalized into a single
    /// representation so generic algorithms can inspect an expression without
    /// branching on its degree-specific storage.
    pub fn items(&self) -> impl Iterator<Item = (Vec<VarRef>, Bias)> {
        self.linear_items()
            .map(|(v, b)| (vec![v], b))
            .chain(self.quadratic_items().map(|(u, v, b)| (vec![u, v], b)))
            .chain(self.higher_order_items())
            .chain(once((Vec::new(), self.offset)))
            .filter(|(_, bias)| *bias != Bias::default())
    }

    /// Iterates over linear contributions as `(variable, bias)` pairs.
    pub fn linear_items(&self) -> impl Iterator<Item = (VarRef, Bias)> {
        self.linear
            .iter()
            .map(|(idx, bias)| (VarRef::new(idx, self.env.clone()), bias))
    }

    /// Iterates over raw linear storage without wrapping variable indices.
    ///
    /// This is primarily useful in lower-level translation and serialization code
    /// that already works with raw indices.
    pub fn raw_linear_items(&self) -> impl Iterator<Item = (u32, Bias)> {
        self.linear.iter()
    }

    /// Iterates over quadratic contributions as `(u, v, bias)` tuples.
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

    /// Iterates over raw quadratic storage without wrapping variable indices.
    pub fn raw_quadratic_items(&self) -> impl Iterator<Item = (u32, u32, Bias)> {
        self.quadratic.iter().flat_map(|q| q.iter_flat())
    }

    /// Iterates over higher-order contributions.
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

    /// Iterates over raw higher-order storage without wrapping variable indices.
    pub fn raw_higher_order_items(&self) -> impl Iterator<Item = (Vec<u32>, Bias)> {
        self.higher_order.iter().flat_map(|q| q.iter_contrib())
    }

    /// Returns the algebraic degree of the expression.
    ///
    /// Degree is derived from the highest-order non-empty term storage:
    /// constants -> `0`, linear -> `1`, quadratic -> `2`, higher-order -> the
    /// stored higher-order degree.
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

    /// Returns `true` if the expression contains no variable-dependent terms.
    pub fn is_constant(&self) -> bool {
        self.degree() == 0
    }

    /// Returns whether the quadratic storage contains any non-zero terms.
    pub fn has_quadratic(&self) -> bool {
        self.quadratic
            .as_ref()
            .map_or_else(|| false, |q| !q.is_empty())
    }

    /// Returns whether the higher-order storage contains any non-zero terms.
    pub fn has_higher_order(&self) -> bool {
        self.higher_order
            .as_ref()
            .map_or_else(|| false, |h| !h.is_empty())
    }

    /// Returns the linear bias for a variable index.
    ///
    /// Missing entries are treated as zero by the underlying storage.
    pub fn linear(&self, idx: VarIdx) -> Bias {
        self.linear[idx]
    }

    /// Returns the quadratic bias for a variable pair.
    pub fn quadratic(&self, u: VarIdx, v: VarIdx) -> Bias {
        self.quadratic
            .as_ref()
            .map_or_else(Bias::default, |q| q[(u, v)])
    }

    /// Returns the higher-order bias for a variable tuple.
    pub fn higher_order(&self, vars: &[VarIdx]) -> Bias {
        self.higher_order
            .as_ref()
            .map_or_else(Bias::default, |h| h[vars])
    }

    /// Returns whether the expression references the given variable.
    pub fn contains(&self, var: &VarRef) -> bool {
        self.vars().map(|v| v.id).contains(&var.id)
    }
}
