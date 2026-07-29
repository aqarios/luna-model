use std::{cmp::Ordering, ops::Neg};

use crate::{Bias, VarIdx};

/// Single sparse `(variable, bias)` contribution.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm {
    idx: VarIdx,
    bias: Bias,
}

impl OneVarTerm {
    pub fn new(idx: VarIdx, bias: Bias) -> Self {
        Self { idx, bias }
    }

    pub fn default(idx: VarIdx) -> Self {
        Self::new(idx, Bias::default())
    }

    pub fn idx(&self) -> VarIdx {
        self.idx
    }

    pub fn bias(&self) -> Bias {
        self.bias
    }

    pub fn bias_ref(&self) -> &Bias {
        &self.bias
    }

    pub fn bias_mut(&mut self) -> &mut Bias {
        &mut self.bias
    }
}

impl Neg for OneVarTerm {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.idx, -self.bias)
    }
}

/// Ordered sparse neighborhood shared by linear and quadratic storage.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Neighborhood(pub Vec<OneVarTerm>);

impl Neighborhood {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns `true` if the sum of all biases is zero.
    pub fn is_zero(&self) -> bool {
        self.iter().map(|(_, b)| b).all(|b| b == Bias::default())
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

    pub fn push_back(&mut self, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.push(OneVarTerm::new(var, bias));
        self
    }

    pub fn push_back_empty(&mut self, var: VarIdx) -> &mut Self {
        self.push_back(var, Bias::default())
    }

    pub fn insert(&mut self, pos: usize, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.insert(pos, OneVarTerm::new(var, bias));
        self
    }

    pub fn insert_empty(&mut self, pos: usize, var: VarIdx) -> &mut Self {
        self.insert(pos, var, Bias::default())
    }

    pub fn find(&self, needle: VarIdx) -> Result<usize, usize> {
        self.0
            .binary_search_by(|term| term.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl std::ops::Deref for Neighborhood {
    type Target = Vec<OneVarTerm>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Neighborhood {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<OneVarTerm> for Neighborhood {
    fn from_iter<T: IntoIterator<Item = OneVarTerm>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<(VarIdx, Bias)> for Neighborhood {
    fn from_iter<T: IntoIterator<Item = (VarIdx, Bias)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(idx, bias)| OneVarTerm::new(idx, bias))
                .collect(),
        )
    }
}

/// Sparse outer entry in a quadratic adjacency list.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TwoVarTerm {
    idx: VarIdx,
    neighborhood: Neighborhood,
}

impl TwoVarTerm {
    pub fn new(idx: VarIdx, neighborhood: Neighborhood) -> Self {
        Self { idx, neighborhood }
    }

    pub fn idx(&self) -> VarIdx {
        self.idx
    }

    pub fn neighborhood(&self) -> &Neighborhood {
        &self.neighborhood
    }

    pub fn neighborhood_mut(&mut self) -> &mut Neighborhood {
        &mut self.neighborhood
    }
}
