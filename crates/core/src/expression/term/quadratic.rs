use super::types::{Neighborhood, TwoVarTerm};
use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};

use std::cmp::Ordering;
use std::ops::{IndexMut, Neg};
use std::{
    ops::{Index, MulAssign},
    sync::LazyLock,
};

// Maybe neighborhood should be it's own type...
static DEFAULT_NEIGHBORHOOD: LazyLock<Neighborhood> = LazyLock::new(|| Neighborhood::default());

#[derive(Debug, Clone)]
pub struct Quadratic {
    adj: Vec<TwoVarTerm>,
}

impl Quadratic {
    pub fn default() -> Self {
        Self {
            adj: Vec::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.adj.len()
    }

    pub fn is_empty(&self) -> bool {
        for (_, n) in self.iter() {
            if !n.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, &Neighborhood)> {
        self.adj.iter().map(|t| (t.idx, &t.neighborhood))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Neighborhood)> {
        self.adj.iter_mut().map(|t| (t.idx, &mut t.neighborhood))
    }

    pub fn iter_flat(&self) -> impl Iterator<Item = (VarIdx, VarIdx, Bias)> {
        self.adj
            .iter()
            .flat_map(|t| t.neighborhood.iter().map(|(n, b)| (t.idx, n, b)))
    }

    pub fn clean(&mut self) {
        self.iter_mut()
            .for_each(|(_, n)| n.retain(|t| t.bias != Bias::default()));
    }

    pub fn push_back_empty(&mut self, idx: VarIdx) -> &mut Self {
        self.adj.push(TwoVarTerm::new(idx, Neighborhood::default()));
        self
    }

    pub fn insert_empty(&mut self, pos: usize, idx: VarIdx) -> &mut Self {
        self.adj
            .insert(pos, TwoVarTerm::new(idx, Neighborhood::default()));
        self
    }

    pub(super) fn find(hay: &[TwoVarTerm], needle: VarIdx) -> Result<usize, usize> {
        hay.binary_search_by(|t| t.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl MulAssign<Bias> for Quadratic {
    fn mul_assign(&mut self, rhs: Bias) {
        self.iter_mut()
            .for_each(|(_, n)| n.iter_mut().for_each(|(_, b)| *b *= rhs));
    }
}

impl Index<VarIdx> for Quadratic {
    type Output = Neighborhood;

    fn index(&self, index: VarIdx) -> &Self::Output {
        let pos = Self::find(&self.adj, index).ok();
        match pos {
            Some(p) => &self.adj[p].neighborhood,
            None => &DEFAULT_NEIGHBORHOOD,
        }
    }
}

impl IndexMut<VarIdx> for Quadratic {
    fn index_mut(&mut self, index: VarIdx) -> &mut Self::Output {
        let pos = Self::find(&self.adj, index).unwrap_or_else(|l| l);
        if pos == self.len() {
            self.push_back_empty(index);
        } else if self.adj[pos].idx != index.into() {
            self.insert_empty(pos, index);
        }
        &mut self.adj[pos].neighborhood
    }
}

impl Index<(VarIdx, VarIdx)> for Quadratic {
    type Output = Bias;
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

fn get_indices(a: VarIdx, b: VarIdx) -> (VarIdx, VarIdx) {
    match a < b {
        true => (a, b),
        false => (b, a),
    }
}
