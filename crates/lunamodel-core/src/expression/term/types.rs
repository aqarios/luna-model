use lunamodel_types::{Bias, VarIdx};

use derive_more::{Deref, DerefMut};

use std::cmp::Ordering;
use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm {
    pub(super) idx: VarIdx,
    pub(super) bias: Bias,
}

impl OneVarTerm {
    pub fn new(idx: VarIdx, bias: Bias) -> Self {
        Self { idx, bias }
    }

    pub fn default(idx: VarIdx) -> Self {
        Self {
            idx,
            bias: Bias::default(),
        }
    }
}

impl Neg for OneVarTerm {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(self.idx, -self.bias)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct Neighborhood(pub Vec<OneVarTerm>);

impl Neighborhood {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn default() -> Self {
        Self(Vec::default())
    }

    pub fn is_zero(&self) -> bool {
        Bias::default() == self.iter().map(|(_, b)| b).sum::<Bias>()
    }

    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, Bias)> {
        self.0.iter().map(|t| (t.idx, t.bias))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Bias)> {
        let mvec: &mut Vec<OneVarTerm> = self.0.as_mut();
        mvec.iter_mut().map(|t| (t.idx, &mut t.bias))
    }

    pub fn push_back(&mut self, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.push(OneVarTerm::new(var, bias));
        self
    }

    pub fn push_back_empty(&mut self, var: VarIdx) -> &mut Self {
        self.0.push(OneVarTerm::new(var, Bias::default()));
        self
    }

    pub fn insert(&mut self, pos: usize, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.insert(pos, OneVarTerm::new(var, bias));
        self
    }

    pub fn insert_empty(&mut self, pos: usize, var: VarIdx) -> &mut Self {
        self.0.insert(pos, OneVarTerm::new(var, Bias::default()));
        self
    }

    pub fn find(&self, needle: VarIdx) -> Result<usize, usize> {
        self.binary_search_by(|t| t.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl FromIterator<OneVarTerm> for Neighborhood {
    fn from_iter<T: IntoIterator<Item = OneVarTerm>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl FromIterator<(VarIdx, Bias)> for Neighborhood {
    fn from_iter<T: IntoIterator<Item = (VarIdx, Bias)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(idx, b)| OneVarTerm::new(idx, b))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TwoVarTerm {
    pub(super) idx: VarIdx,
    pub(super) neighborhood: Neighborhood,
}

impl TwoVarTerm {
    pub fn new(idx: VarIdx, neighborhood: Neighborhood) -> Self {
        Self { idx, neighborhood }
    }

    pub fn empty(idx: VarIdx) -> Self {
        Self {
            idx,
            neighborhood: Neighborhood::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.neighborhood.is_empty()
    }

    pub fn push(&mut self, neighbor: OneVarTerm) -> &mut Self {
        self.neighborhood.push(neighbor);
        self
    }

    pub fn last(&self) -> Option<&OneVarTerm> {
        self.neighborhood.last()
    }
}
