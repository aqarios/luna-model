//! Shared helper types used by sparse term storage implementations.

use lunamodel_types::{Bias, VarIdx};

use derive_more::{Deref, DerefMut};

use std::cmp::Ordering;
use std::ops::Neg;

/// Single `(variable, bias)` contribution used inside sparse neighborhoods.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm {
    pub(super) idx: VarIdx,
    pub(super) bias: Bias,
}

impl OneVarTerm {
    /// Creates a one-variable term.
    pub fn new(idx: VarIdx, bias: Bias) -> Self {
        Self { idx, bias }
    }

    /// Creates a zero-bias term for a variable.
    pub fn default(idx: VarIdx) -> Self {
        Self {
            idx,
            bias: Bias::default(),
        }
    }
}

impl Neg for OneVarTerm {
    type Output = Self;

    /// Negates the stored bias while keeping the variable index unchanged.
    fn neg(self) -> Self::Output {
        Self::new(self.idx, -self.bias)
    }
}

/// Ordered sparse neighborhood of one-variable terms.
///
/// `Neighborhood` is the common storage used by both linear terms and the inner
/// neighborhoods of quadratic terms. Entries are kept ordered by variable index
/// so binary search can be used for lookup and insertion.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct Neighborhood(pub Vec<OneVarTerm>);

impl Neighborhood {
    /// Creates an empty neighborhood.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates an empty neighborhood.
    pub fn default() -> Self {
        Self(Vec::default())
    }

    /// Returns `true` if the sum of all biases is zero.
    pub fn is_zero(&self) -> bool {
        Bias::default() == self.iter().map(|(_, b)| b).sum::<Bias>()
    }

    /// Iterates over `(variable, bias)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, Bias)> {
        self.0.iter().map(|t| (t.idx, t.bias))
    }

    /// Iterates mutably over `(variable, bias)` pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Bias)> {
        let mvec: &mut Vec<OneVarTerm> = self.0.as_mut();
        mvec.iter_mut().map(|t| (t.idx, &mut t.bias))
    }

    /// Appends a new term to the back without checking ordering.
    ///
    /// Callers are expected to maintain the sorted invariant themselves.
    pub fn push_back(&mut self, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.push(OneVarTerm::new(var, bias));
        self
    }

    /// Appends a zero-bias term to the back.
    pub fn push_back_empty(&mut self, var: VarIdx) -> &mut Self {
        self.0.push(OneVarTerm::new(var, Bias::default()));
        self
    }

    /// Inserts a term at an already computed position.
    pub fn insert(&mut self, pos: usize, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.insert(pos, OneVarTerm::new(var, bias));
        self
    }

    /// Inserts a zero-bias term at an already computed position.
    pub fn insert_empty(&mut self, pos: usize, var: VarIdx) -> &mut Self {
        self.0.insert(pos, OneVarTerm::new(var, Bias::default()));
        self
    }

    /// Binary-searches for a variable index in the sorted neighborhood.
    pub fn find(&self, needle: VarIdx) -> Result<usize, usize> {
        self.binary_search_by(|t| t.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl FromIterator<OneVarTerm> for Neighborhood {
    /// Collects one-variable terms into a neighborhood without additional normalization.
    fn from_iter<T: IntoIterator<Item = OneVarTerm>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl FromIterator<(VarIdx, Bias)> for Neighborhood {
    /// Collects `(variable, bias)` pairs into a neighborhood.
    fn from_iter<T: IntoIterator<Item = (VarIdx, Bias)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(idx, b)| OneVarTerm::new(idx, b))
                .collect(),
        )
    }
}

/// Sparse outer entry in a quadratic adjacency list.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TwoVarTerm {
    pub(super) idx: VarIdx,
    pub(super) neighborhood: Neighborhood,
}

impl TwoVarTerm {
    /// Creates a quadratic adjacency entry for one outer variable.
    pub fn new(idx: VarIdx, neighborhood: Neighborhood) -> Self {
        Self { idx, neighborhood }
    }

    // pub fn empty(idx: VarIdx) -> Self {
    //     Self {
    //         idx,
    //         neighborhood: Neighborhood::default(),
    //     }
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.neighborhood.is_empty()
    // }

    // pub fn push(&mut self, neighbor: OneVarTerm) -> &mut Self {
    //     self.neighborhood.push(neighbor);
    //     self
    // }

    // pub fn last(&self) -> Option<&OneVarTerm> {
    //     self.neighborhood.last()
    // }
}
