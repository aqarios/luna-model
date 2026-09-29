//! Shared helper types used by sparse term storage implementations.

use lunamodel_types::{Bias, VarIdx};

use std::cmp::Ordering;
use std::ops::{Index, IndexMut, Neg};

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
///
/// Entries whose bias is exactly zero are *storage* details: they are kept so
/// the sorted positions stay stable, but they are not part of the term the
/// neighborhood represents. Accordingly [`len`](Self::len),
/// [`is_empty`](Self::is_empty) and [`iter`](Self::iter) all ignore them. The
/// `storage_*` accessors expose the raw backing vector and must be used for
/// anything positional, because [`find`](Self::find) returns raw indices.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
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
        self.iter().map(|(_, b)| b).all(|b| b == Bias::default())
    }

    /// Returns the number of non-zero contributions.
    ///
    /// This counts exactly what [`iter`](Self::iter) yields. Use
    /// [`storage_len`](Self::storage_len) when a raw index bound is needed.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns `true` if there is no non-zero contribution.
    ///
    /// This agrees with [`iter`](Self::iter). Use
    /// [`storage_is_empty`](Self::storage_is_empty) to ask whether the backing
    /// vector holds no entries at all.
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    /// Returns the number of entries in the backing vector, zero biases included.
    ///
    /// This is the bound that [`find`](Self::find) positions are relative to.
    pub fn storage_len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the backing vector holds no entries at all.
    pub fn storage_is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Drops entries the predicate rejects from the backing vector.
    pub fn retain(&mut self, f: impl FnMut(&OneVarTerm) -> bool) {
        self.0.retain(f);
    }

    /// Iterates over `(variable, bias)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, Bias)> {
        self.0
            .iter()
            .filter_map(|t| match t.bias != Bias::default() {
                true => Some((t.idx, t.bias)),
                false => None,
            })
    }

    /// Iterates mutably over `(variable, bias)` pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Bias)> {
        let mvec: &mut Vec<OneVarTerm> = self.0.as_mut();
        mvec.iter_mut()
            .filter_map(|t| match t.bias != Bias::default() {
                true => Some((t.idx, &mut t.bias)),
                false => None,
            })
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
        self.0
            .binary_search_by(|t| t.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

/// Raw positional access into the backing vector.
///
/// Positions are the ones produced by [`Neighborhood::find`], so they address
/// stored entries including zero biases.
impl Index<usize> for Neighborhood {
    type Output = OneVarTerm;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.0[pos]
    }
}

/// Raw mutable positional access into the backing vector.
impl IndexMut<usize> for Neighborhood {
    fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        &mut self.0[pos]
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

#[cfg(test)]
mod tests {
    use super::*;

    /// `len`, `is_empty` and `iter` must agree, so callers cannot pick a
    /// counting method that silently disagrees with the one they iterate with.
    #[test]
    fn len_and_is_empty_track_iter() {
        let mut n = Neighborhood::new();
        n.push_back(0, 1.0);
        n.push_back_empty(1); // cancelled-out contribution, still stored
        n.push_back(2, 2.0);

        assert_eq!(n.len(), n.iter().count());
        assert_eq!(n.len(), 2);
        assert!(!n.is_empty());

        // the raw accessors still see the stored zero
        assert_eq!(n.storage_len(), 3);
        assert!(!n.storage_is_empty());
    }

    /// A neighborhood holding nothing but zero biases is empty as a term even
    /// though its backing storage is not.
    #[test]
    fn all_zero_neighborhood_is_empty_but_stored() {
        let mut n = Neighborhood::new();
        n.push_back_empty(0);
        n.push_back_empty(1);

        assert_eq!(n.len(), 0);
        assert!(n.is_empty());
        assert!(n.is_zero());
        assert_eq!(n.storage_len(), 2);
    }

    /// `find` returns raw positions, so they must stay valid against
    /// `storage_len` rather than the filtered `len`.
    #[test]
    fn find_positions_are_raw() {
        let mut n = Neighborhood::new();
        n.push_back_empty(0);
        n.push_back(1, 5.0);

        let pos = n.find(1).unwrap();
        assert_eq!(pos, 1);
        assert_eq!(n[pos].bias, 5.0);
        assert!(pos < n.storage_len());
    }
}
