//! Sparse storage for quadratic expression terms.

use crate::traits::Editable;

use super::types::{Neighborhood, TwoVarTerm};
use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};

use std::cmp::Ordering;
use std::ops::{AddAssign, IndexMut, Mul, Neg};
use std::{
    ops::{Index, MulAssign},
    sync::LazyLock,
};

static DEFAULT_NEIGHBORHOOD: LazyLock<Neighborhood> = LazyLock::new(Neighborhood::default);

/// Sparse symmetric storage for quadratic expression terms.
///
/// Quadratic terms are stored as an adjacency list keyed by the smaller of the
/// two variable indices. Each outer entry then stores a sorted neighborhood of
/// the larger partner indices. This keeps lookup and insertion cheap while
/// avoiding duplicate storage for `(u, v)` and `(v, u)`.
#[derive(Default, Debug, Clone)]
pub struct Quadratic {
    adj: Vec<TwoVarTerm>,
}
impl Editable for Quadratic {}

impl Quadratic {
    /// Returns the number of outer adjacency entries.
    pub fn len(&self) -> usize {
        self.adj.len()
    }

    /// Returns `true` if no non-zero quadratic terms are stored.
    pub fn is_empty(&self) -> bool {
        for (_, n) in self.iter() {
            if !n.is_empty() {
                return false;
            }
        }
        true
    }

    /// Returns `true` if the sum of all stored quadratic biases is zero.
    pub fn is_zero(&self) -> bool {
        self.iter_flat().map(|(_, _, b)| b).sum::<Bias>() == 0.0
    }

    /// Iterates over the outer adjacency entries.
    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, &Neighborhood)> {
        self.adj.iter().map(|t| (t.idx, &t.neighborhood))
    }

    /// Iterates mutably over the outer adjacency entries.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Neighborhood)> {
        self.adj.iter_mut().map(|t| (t.idx, &mut t.neighborhood))
    }

    /// Flattens the sparse adjacency list into `(u, v, bias)` triples.
    pub fn iter_flat(&self) -> impl Iterator<Item = (VarIdx, VarIdx, Bias)> {
        self.adj
            .iter()
            .flat_map(|t| t.neighborhood.iter().map(|(n, b)| (t.idx, n, b)))
    }

    /// Removes explicitly stored zero entries from all neighborhoods.
    pub fn clean(&mut self) {
        self.iter_mut()
            .for_each(|(_, n)| n.retain(|t| t.bias != Bias::default()));
    }

    /// Appends an empty outer adjacency entry.
    pub fn push_back_empty(&mut self, idx: VarIdx) -> &mut Self {
        self.adj.push(TwoVarTerm::new(idx, Neighborhood::default()));
        self
    }

    /// Inserts an empty outer adjacency entry at a known sorted position.
    pub fn insert_empty(&mut self, pos: usize, idx: VarIdx) -> &mut Self {
        self.adj
            .insert(pos, TwoVarTerm::new(idx, Neighborhood::default()));
        self
    }

    /// Binary-searches the outer adjacency list by variable index.
    pub(super) fn find(hay: &[TwoVarTerm], needle: VarIdx) -> Result<usize, usize> {
        hay.binary_search_by(|t| t.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl MulAssign<Bias> for Quadratic {
    /// Scales all stored quadratic biases.
    fn mul_assign(&mut self, rhs: Bias) {
        if rhs == Bias::default() {
            *self = Self::default();
            return;
        }
        self.iter_mut()
            .for_each(|(_, n)| n.iter_mut().for_each(|(_, b)| *b *= rhs));
    }
}

impl Mul<Bias> for Quadratic {
    type Output = Self;

    /// Returns a scaled copy of the quadratic storage.
    fn mul(mut self, rhs: Bias) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Index<VarIdx> for Quadratic {
    type Output = Neighborhood;

    /// Returns the neighborhood for an outer variable, defaulting to an empty one.
    fn index(&self, index: VarIdx) -> &Self::Output {
        let pos = Self::find(&self.adj, index).ok();
        match pos {
            Some(p) => &self.adj[p].neighborhood,
            None => &DEFAULT_NEIGHBORHOOD,
        }
    }
}

impl IndexMut<VarIdx> for Quadratic {
    /// Returns mutable access to the neighborhood for an outer variable.
    fn index_mut(&mut self, index: VarIdx) -> &mut Self::Output {
        let pos = Self::find(&self.adj, index).unwrap_or_else(|l| l);
        if pos == self.len() {
            self.push_back_empty(index);
        } else if self.adj[pos].idx != index {
            self.insert_empty(pos, index);
        }
        &mut self.adj[pos].neighborhood
    }
}

impl Index<(VarIdx, VarIdx)> for Quadratic {
    type Output = Bias;

    /// Returns the quadratic bias for a variable pair, defaulting to zero when absent.
    fn index(&self, index: (VarIdx, VarIdx)) -> &Self::Output {
        let (outer, inner) = get_indices(index.0, index.1);
        let pos = Self::find(&self.adj, outer).ok();
        let nei = match pos {
            Some(p) => &self.adj[p].neighborhood,
            None => &DEFAULT_NEIGHBORHOOD,
        };
        let pos = nei.find(inner).ok();
        match pos {
            Some(p) => &nei[p].bias,
            None => &DEFAULT_BIAS,
        }
    }
}

impl IndexMut<(VarIdx, VarIdx)> for Quadratic {
    /// Returns mutable access to the quadratic bias for a variable pair.
    fn index_mut(&mut self, index: (VarIdx, VarIdx)) -> &mut Self::Output {
        let (outer, inner) = get_indices(index.0, index.1);
        let nei = &mut self[outer];
        let pos = nei.find(inner).unwrap_or_else(|l| l);
        if pos == nei.len() {
            nei.push_back_empty(inner);
        } else if nei[pos].idx != inner {
            nei.insert_empty(pos, inner);
        }
        &mut nei[pos].bias
    }
}

impl PartialEq for Quadratic {
    /// Compares two quadratic storages while treating implicit and explicit zeros equally.
    fn eq(&self, other: &Self) -> bool {
        // need to check both, since one might contain 0.0 explicitly while the other only
        // implicitly.

        // check that all items in self are in other,
        for (u, v, b) in self.iter_flat() {
            if b != other[(u, v)] {
                return false;
            }
        }
        // check that all items in other are in self,
        for (u, v, b) in other.iter_flat() {
            if b != self[(u, v)] {
                return false;
            }
        }
        true
    }
}

impl Neg for Quadratic {
    type Output = Self;

    /// Negates every stored quadratic bias.
    fn neg(self) -> Self::Output {
        Self {
            adj: self
                .adj
                .iter()
                .map(|t| {
                    TwoVarTerm::new(
                        t.idx,
                        t.neighborhood
                            .iter()
                            .map(|(idx, bias)| (idx, -bias))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

/// Canonicalizes a quadratic variable pair so `(u, v)` and `(v, u)` share storage.
fn get_indices(a: VarIdx, b: VarIdx) -> (VarIdx, VarIdx) {
    match a < b {
        true => (a, b),
        false => (b, a),
    }
}

impl AddAssign<(VarIdx, VarIdx, Bias)> for Quadratic {
    /// Adds a single quadratic contribution.
    fn add_assign(&mut self, rhs: (VarIdx, VarIdx, Bias)) {
        let (u, v, b) = rhs;
        if b == Bias::default() {
            return;
        }
        self[(u, v)] += b
    }
}

impl AddAssign<&Quadratic> for Quadratic {
    /// Adds all contributions from another quadratic storage.
    fn add_assign(&mut self, rhs: &Quadratic) {
        for (u, v, bias) in rhs.iter_flat() {
            *self += (u, v, bias)
        }
    }
}

impl AddAssign<Quadratic> for Quadratic {
    /// Adds all contributions from another quadratic storage.
    fn add_assign(&mut self, rhs: Quadratic) {
        self.add_assign(&rhs);
    }
}
